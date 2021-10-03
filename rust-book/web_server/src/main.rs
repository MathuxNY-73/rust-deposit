use std::io::prelude::*;
use std::io::{Result};
use std::net::{TcpListener, TcpStream };
use std::fs;
use std::thread;
use std::time::Duration;
use web_server::ThreadPool;
use ctrlc;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    listener.set_nonblocking(true).expect("Cannot set non-blocking");

    let pool = ThreadPool::new(4);

    let is_term = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let is_term_handle = is_term.clone();
    ctrlc::set_handler(move || {
        println!("received Ctrl+C!");
        is_term_handle.swap(true, std::sync::atomic::Ordering::Relaxed);
    }).expect("Could not set handler!");

    for stream in listener.incoming() {
        let should_terminate = is_term.load(std::sync::atomic::Ordering::Acquire);
        if should_terminate {
            break;
        }

        match stream {
            Ok(stream) => {

                pool.execute(|| {
                if let Err(e) = handle_connection(stream) {
                   panic!("{}", e);
                }
                });
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => panic!("Encountered IO Error {}", e),
        }
    }

    println!("Shutting down the server");
}

fn handle_connection(mut stream: TcpStream) -> Result<()> {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    Result::Ok(())
}