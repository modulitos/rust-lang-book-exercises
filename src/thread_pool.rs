use std::thread;
pub struct ThreadPool {
    threads: usize,
}

impl ThreadPool {
    pub fn new(threads: usize) -> Self {
        Self { threads }
    }

    pub fn execute<F, T>(&self, closure: F)
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
    {
        // println!("executing!");
        thread::spawn(closure);
    }
}
