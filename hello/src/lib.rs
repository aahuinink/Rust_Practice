use std::{
    fs, io::{prelude::*, BufReader}, net::TcpStream, sync::{mpsc::{self, Receiver}, Arc, Mutex}, thread::{self, JoinHandle}
};

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    thread: JoinHandle<Arc<Mutex<Receiver<Job>>>>,
}

impl Worker {
    /// Creates a new worker struct that can wait for jobs sent to the thread pool
    /// 
    /// 
    fn new(id: usize, reciever: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread: JoinHandle<Arc<Mutex<Receiver<Box<dyn FnOnce() + Send>>>>> = thread::spawn(move || loop {
            let job: Box<dyn FnOnce() + Send> = reciever.lock().unwrap().recv().unwrap();
            println!("Worker {id} got a job; executing.");

            job();
        });
        Worker { id, thread}
    }
}

pub struct ThreadPool {
    sender: mpsc::Sender<Job>,
    workers: Vec<Worker>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    /// 
    /// The size is the number of threads in the pool.
    /// 
    ///  # Panics
    /// 
    /// The 'new' function panics if the size is 0.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let mut workers: Vec<Worker> = Vec::with_capacity(4);

        let (sender, reciever) = mpsc::channel();

        let reciever = Arc::new(Mutex::new(reciever));

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&reciever)));
        }

        ThreadPool { sender, workers }
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
        {
            let job = Box::new(f);

            self.sender.send(job).unwrap();
        }
}


pub fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP /1.1 200 OK", "hello.html")
    } else {
        ("HTTP /1.1 404 NOT FOUND", "404.html")
    };
    let contents: String = fs::read_to_string(filename).unwrap();
    let length: usize = contents.len();
    let response: String = 
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}