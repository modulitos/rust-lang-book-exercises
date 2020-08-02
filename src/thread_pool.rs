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

        Worker {
            id,
            handle: thread::spawn(move || {
                loop {

                    // Using an Arc<Mutex>> because taking a job off the channel queue involves
                    // mutating the receiver, so the threads need a safe way to share and modify
                    // receiver; otherwise, we might get race conditions.

                    let job = receiver.lock().unwrap().recv().unwrap();

                    println!("thread {} received a new job.", id);
                    job();
                    println!("thread {} job finished.", id);
                }
            }),
        }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

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

            // For each new worker, we clone the Arc to bump the reference count so the workers can
            // share ownership of the receiving end.

            workers.push(Worker::new(i, Arc::clone(&receiver)));
            // workers.push(Worker::new(i, receiver.clone()));

            println!("new worker has been started with id: {}", i);
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, job: F)
    where
        // The type for F is taken from the method signature of thread::spawn() here:
        // https://doc.rust-lang.org/std/thread/fn.spawn.html
        F: FnOnce(),
        F: Send + 'static,
    {
        // Send the job down the sending end of the channel.

        self.sender
            .send(Box::new(job))

            // We’re calling expect on send for the case that sending fails. This might happen if,
            // for example, we stop all our threads from executing, meaning the receiving end has
            // stopped receiving new messages. Currently, we can’t stop our threads from executing:
            // our threads continue executing as long as the pool exists. The reason we use unwrap
            // is that we know the failure case won’t happen, but the compiler doesn’t know that.

            .expect("Failed to send job to channel consumer");
    }
}
