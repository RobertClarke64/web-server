use std::fmt;
use std::error::Error;
use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};
use log::debug;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

/// Pool of threads, queuing tasks and sending them to available
/// threads to process
impl ThreadPool {

    /// Create a new ThreadPool
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Errors
    ///
    /// The `build` function will error with [struct@PoolCreationError] if number
    /// of threads is zero.
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size < 1  {
            return Err(PoolCreationError::new("Thread pool must contain at least 1 thread"));
        }

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Ok(ThreadPool { workers, sender })
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            debug!("Worker {id} got a job; executing.");

            job();
        });

        Worker { id, thread }
    }
}

/// Error while trying to create a new thread pool
#[derive(Debug)]
pub struct PoolCreationError {
    message: String,
}

impl PoolCreationError {
    fn new(msg: &str) -> PoolCreationError {
        PoolCreationError { message: msg.to_string() }
    }
}

impl fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for PoolCreationError {}
