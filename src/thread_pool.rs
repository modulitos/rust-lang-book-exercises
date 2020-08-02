use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

enum WorkerMessage {
    DoWork(Job),
    Terminate,
}

// Each Worker stores a single JoinHandle<()> instance. Each worker has an id so we can distinguish
// between the different workers in the pool when logging or debugging.

struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    // Using an Arc<Mutex>> because taking a job off the channel queue involves
    // mutating the receiver, so the threads need a safe way to share and modify
    // receiver; otherwise, we might get race conditions.

    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<WorkerMessage>>>) -> Self {
        let thread = thread::spawn(move || {
            loop {
                // Here, we first call lock on the receiver to acquire the mutex, and then we
                // call unwrap to panic on any errors. Acquiring a lock might fail if the mutex
                // is in a poisoned state, which can happen if some other thread panicked while
                // holding the lock rather than releasing the lock. In this situation, calling
                // unwrap to have this thread panic is the correct action to take.

                // Note that .lock acquires a mutex, blocking the current thread until it is
                // able to do so.

                // If we get the lock on the mutex, we call recv to receive a message from the
                // channel. A final unwrap moves past any errors here as well, which might occur if
                // the thread holding the sending side of the channel has shut down, similar to how
                // the send method returns Err if the receiving side shuts down.

                // The call to recv blocks, so if there is no message yet, the current thread will
                // wait until a message becomes available. The Mutex<T> ensures that only one Worker
                // thread at a time is trying to request a message.

                match receiver.lock().unwrap().recv().unwrap() {
                    WorkerMessage::DoWork(job) => {
                        // Note that the temporary MutexGuard returned from the lock method is dropped
                        // as soon as the "let job =" statement ends. This ensures that the lock is held
                        // during the call to recv, but it is released before the call to job(),
                        // allowing multiple requests to be serviced concurrently.

                        // AKA, we are freeing up other threads to access the receiver *before* the job
                        // is run.

                        println!("thread {} received a new job.", id);
                        job();
                        println!("thread {} job finished.", id);
                    }
                    WorkerMessage::Terminate => {
                        println!("worker is terminating");
                        break;
                    }
                }
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<WorkerMessage>,
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

    /// Takes a closure of code to run and sends it to the already running thread for execution.

    pub fn execute<F>(&self, job: F)
    where
        // The type for F is taken from the method signature of thread::spawn() here:
        // https://doc.rust-lang.org/std/thread/fn.spawn.html
        F: FnOnce(),
        F: Send + 'static,
    {
        // Send the job down the sending end of the channel.

        self.sender
            .send(WorkerMessage::DoWork(Box::new(job)))
            // We’re calling expect on send for the case that sending fails. This might happen if,
            // for example, we stop all our threads from executing, meaning the receiving end has
            // stopped receiving new messages. Currently, we can’t stop our threads from executing:
            // our threads continue executing as long as the pool exists. The reason we use unwrap
            // is that we know the failure case won’t happen, but the compiler doesn’t know that.
            .expect("Failed to send job to channel consumer");
    }
}

/// When the pool is dropped, our threads should all join to make sure they finish their work.
///
/// When the ThreadPool goes out of scope at the end of main, its Drop implementation kicks in, and
/// the pool tells all workers to terminate. The workers each print a message when they see the
/// terminate message, and then the thread pool calls join to shut down each worker thread.

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("drop trait!");

        // We’re now iterating over the workers twice: once to send one Terminate message for each
        // worker and once to call join on each worker’s thread. If we tried to send a message and
        // join immediately in the same loop, we couldn’t guarantee that the worker in the current
        // iteration would be the one to get the message from the channel.

        // So, we can be sure that if we send the same number of terminate messages as there are
        // workers, each worker will receive a terminate message before join is called on its
        // thread.

        self.workers.iter().for_each(|_| {
            self.sender
                .send(WorkerMessage::Terminate)
                .expect("Failed to send job to channel consumer")
        });

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                // Block the main thread, and wait for the associated thread to finish.
                thread.join().unwrap();
            }
        }
    }
}
