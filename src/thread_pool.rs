use std::thread;
use std::thread::JoinHandle;

pub struct ThreadPool {
    threads: Vec<JoinHandle<()>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let mut threads = Vec::with_capacity(size);

        for _ in 0..size {
            // create some threads and store them in the vector
            threads.push(thread::spawn(|| loop {}))
        }

        ThreadPool { threads }
    }

    pub fn execute<F, T>(&self, closure: F)
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
    {
        // println!("executing!");

        // TODO: instead of spawning a thread here, we should pass in the closure to a thread from
        // our threadpool.

        thread::spawn(closure);
    }
}
