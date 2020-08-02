use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

// Each Worker stores a single JoinHandle<()> instance. Each worker has an id so we can distinguish
// between the different workers in the pool when logging or debugging.

struct Worker {
    id: usize,
    handle: JoinHandle<()>,
}

impl Worker {
    // Takes a closure of code to run and sends it to the already running thread for execution.

    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {

        // taking a job off the channel queue involves mutating the receiver, so the threads need a safe way to share and modify receiver; otherwise, we might get race conditions

        Worker {
            id,
            handle: thread::spawn(|| {
                receiver;
                loop {}
            }),
        }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

struct Job {}


impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);
        let (sender, receiver) = mpsc::channel();

        // the channel implementation that Rust provides is multiple producer, single consumer. This
        // means we can’t just clone the consuming end of the channel to fix this code. Even if we
        // could, that is not the technique we would want to use; instead, we want to distribute the
        // jobs across threads by sharing the single receiver among all the workers.

        let receiver = Arc::new(Mutex::new(receiver));

        for i in 0..size {
            // create some threads and store them in the vector
            workers.push(Worker::new(i, Arc::clone(&receiver)));
            // workers.push(Worker::new(i, receiver.clone()));
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
