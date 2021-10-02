use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::{Arc, Mutex};


pub struct ThreadPool {
    tx: Sender<Message>,
    workers: Vec<Worker>
}

impl ThreadPool {
    // Try to write a version where new return Result<ThreadPool, PoolCreationError>
    pub fn new(nb_threads: usize) -> ThreadPool {
        assert!(nb_threads > 0);

        let mut workers = Vec::with_capacity(nb_threads);
        let (tx, rx) = mpsc::channel();

        let rx = Arc::new(Mutex::new(rx));

        for id in 0..nb_threads {
            workers.push(Worker::new(id, rx.clone()));
        }

        ThreadPool {
            tx,
            workers
        }
    }

    pub fn execute<F>(&self, action: F) 
        where F: FnOnce() + Send + 'static {
            let job = Box::new(action);

            self.tx.send(Message::Execute(job)).unwrap();
        }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &self.workers {
            self.tx.send(Message::Terminate).unwrap();
        }
        
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                println!("Shutting down worker {}", worker.id);
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, rx: Arc<Mutex<Receiver<Message>>>) -> Worker {

        let thread = thread::spawn(move || loop {
                let message = rx.lock().unwrap().recv().unwrap();
            
                match message {
                    Message::Execute(job) => {
                        println!("Worker {} executing the job", id);
                        job();
                    }
                    Message::Terminate => {
                        println!("Telling worker {} to shut down", id);
                        break;
                    }
                }
                
            }
        );
        Worker { id, thread: Some(thread) }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    Execute(Job),
    Terminate
}