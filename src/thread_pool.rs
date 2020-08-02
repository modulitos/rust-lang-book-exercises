use std::thread;
use std::thread::JoinHandle;

struct Worker {
    id: usize,
    handle: JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Self {
        Worker {
            id,
            handle: thread::spawn(|| loop {}),
        }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);

        for i in 0..size {
            // create some threads and store them in the vector
            workers.push(Worker::new(i))
        }

        ThreadPool { workers }
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
