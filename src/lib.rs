use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc::{self, Receiver, Sender};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool{
    pub fn new(size: usize) -> ThreadPool{
        let (sender,receiver) = mpsc::channel();
        let mut workers = Vec::with_capacity(size);
        let receiver = Arc::new(Mutex::new(receiver));
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }        
        ThreadPool {workers, sender}
    }

    pub fn execute<F>(&self, f: F)
    where 
        F: FnOnce() -> () + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    } 
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(|| {
            receiver;
        });
        Worker{id, thread}
    }
}

