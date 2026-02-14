#![allow(unused)]

use std::thread;

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
}

pub enum PoolCreationError {
    ZeroRequested(String),
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert_ne!(size, 0);

        let mut threads = Vec::with_capacity(size);

        for _ in 0..size {}

        ThreadPool { threads }
    }

    /// Create a new ThreadPool.
    /// Could return Error if the size is zero.
    ///
    /// The size is the number of threads in the pool.
    fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size == 0 {
            Err(PoolCreationError::ZeroRequested(String::from(
                "Zero size was requested",
            )))
        } else {
            let mut threads = Vec::with_capacity(size);

            for _ in 0..size {}

            Ok(ThreadPool { threads })
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}
