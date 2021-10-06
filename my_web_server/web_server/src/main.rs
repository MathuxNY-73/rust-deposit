use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web::http::header::ContentType;
use actix_web::web::Bytes;

use image::io::Reader as ImageReader;
use image::{DynamicImage, ImageResult};

/*
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            // In a loop, read data from the socket and write the data back.
            loop {
                let n = match socket.read(&mut buf).await {
                    // socket closed
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                // Write the data back
                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}
*/

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/picture")]
async fn picture() -> impl Responder {
    let img_res = read_image("./data/my_image.jpg");
    if let Err(e) = img_res {
        panic!("Could not read image with error {}", e);
    }
    let img = img_res.unwrap();

    HttpResponse::Ok()
        .set(ContentType::png())
        .body(Bytes::from(img.to_bytes()))
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(picture)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

fn read_image(path: &str) -> ImageResult<DynamicImage> {
    let img = ImageReader::open(path)?.decode()?;
    ImageResult::Ok(img)
}