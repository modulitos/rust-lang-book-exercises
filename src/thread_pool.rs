use std::sync::mpsc;
use std::thread;
use std::thread::JoinHandle;

// Each Worker stores a single JoinHandle<()> instance. Each worker has an id so we can distinguish
// between the different workers in the pool when logging or debugging.

struct Worker {
    id: usize,
    handle: JoinHandle<Job>,
}

struct Job {}

impl Worker {
    // Takes a closure of code to run and sends it to the already running thread for execution.

    fn new(id: usize) -> Self {
        Worker {
            id,
            handle: thread::spawn(|| loop {}),
        }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<()>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);
        let (sender, rx) = mpsc::channel();

        for i in 0..size {
            // create some threads and store them in the vector
            workers.push(Worker::new(i))
        }

        ThreadPool { workers, sender }
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
