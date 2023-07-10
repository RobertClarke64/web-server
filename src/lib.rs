use std::fmt;
use std::error::Error;
use std::thread;

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
}

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
            return Err(PoolCreationError::new("Cannot create thread pool of size 0"));
        }

        let mut threads = Vec::with_capacity(size);

        for _ in 0..size {
            // create some threads and store them in the vector
        }

        Ok(ThreadPool { threads })
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        
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
