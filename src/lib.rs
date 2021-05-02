use std::{thread, usize};
use std::sync::{mpsc, Arc, Mutex};

pub struct Worker {
    id: usize,
    workerThread: thread::JoinHandle<()>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) ->Self {
        Self {
            id,
            workerThread: thread::spawn(move || {
                let job = receiver.lock().unwrap().recv().unwrap();
                println!("Worker {} got a job; executing.", id);
                job();
            })
        }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool{
    threads: Vec<Worker>,
    sender: mpsc::Sender<Job>
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel::<Job>();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut threads = Vec::with_capacity(size);

        for n in 0..size {
            threads.push(Worker::new(n as usize, Arc::clone(&receiver)));
        }

        Self{
            threads,
            sender,
        }
    }

    pub fn execute<F>(&self, f: F)
    where 
        F: FnOnce()+ Send + 'static 
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}