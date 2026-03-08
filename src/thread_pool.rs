use std::{
    sync::{
        Arc, Mutex,
        mpsc::{self, Receiver},
    },
    thread::{self, JoinHandle},
};

type Job = Box<dyn FnOnce() + Send + 'static>;

#[allow(unused)]
#[derive(Debug)]
struct Worker {
    id: u8,
    thread: JoinHandle<()>,
}

impl Worker {
    fn new(id: u8, receiver: Arc<Mutex<Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv();
                match message {
                    Ok(job) => {
                        job();
                    }
                    Err(e) => {
                        eprintln!("worker {id} failed: {e}");
                        break;
                    }
                }
            }
        });
        Self { id, thread }
    }
}

#[allow(unused)]
#[derive(Debug)]
pub struct ThreadPool {
    size: u8,
    sender: Option<mpsc::Sender<Job>>,
    workers: Vec<Worker>,
}

impl ThreadPool {
    pub fn new(size: u8) -> Self {
        let (sender, receiver) = mpsc::channel();
        let sender = Some(sender);
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::new();

        for id in 0..size {
            workers.push(Worker::new(id, receiver.clone()));
        }

        Self { size, sender, workers }
    }
    pub fn excute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).expect("failed to send job");
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in self.workers.drain(..) {
            worker.thread.join().unwrap();
        }
    }
}
