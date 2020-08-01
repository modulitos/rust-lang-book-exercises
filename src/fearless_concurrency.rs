pub mod threads {
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    use std::boxed;
    use std::error;
    use std::result;

    type Error = boxed::Box<dyn error::Error>;
    type Result<T, E = Error> = result::Result<T, E>;

    pub fn test() {
        thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    }
    pub fn test_join_handle() -> Result<()> {
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        // TODO: how to replace this with the ? operator??? Updating our Error type?
        handle.join().unwrap();
        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
        Ok(())
    }
    pub fn test_moving_data() {
        let v = vec![1, 2, 3];

        let handle = thread::spawn(move || {
            println!("Here's a vector: {:?}", v);
        });

        handle.join().unwrap();
    }

    pub fn test_channels() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let s = String::from("hi");
            tx.send(s).unwrap();
        });
        let received = rx.recv().unwrap();
        println!("Got: {}", received);
    }

    pub fn test_multiple_messages() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        // instead of calling `recv` on rx, we're treating it as an iterator. Therefore, it will
        // wait for values until the channel is closed, which will end the iterator

        for received in rx {
            println!("Got: {}", received);
        }
    }

    pub fn test_multiple_producers() {
        let (tx, rx) = mpsc::channel();

        // let tx1 = mpsc::Sender::clone(&tx);
        let tx1 = tx.clone();
        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        thread::spawn(move || {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        for received in rx {
            println!("Got: {}", received);
        }
    }

    // 2 Rules with mutexes:
    //
    //  * You must attempt to acquire the lock before using the data.
    //
    //  * When you’re done with the data that the mutex guards, you must unlock the data so other
    //  threads can acquire the lock.
    //
    // Eg: like sharing a single microphone at a panel discussion.
    pub mod mutexes {
        use super::*;
        use std::sync::{Arc, Mutex};
        // use create::fearless_concurrency::threads::*;

        pub fn basic() {
            let m = Mutex::new(5);

            {
                // blocks the thread until lock is freed.
                // If the thread holding the lock has panicked, then lock() returns an Err

                let mut num = m.lock().unwrap();
                // MutexGuard implements Deref:
                *num = 6;
                // dropping the MutexGuard releases the lock automatically
            }

            println!("m = {:?}", m);
        }

        // Using this strategy, you can divide a calculation into independent parts, split those
        // parts across threads, and then use a Mutex<T> to have each thread update the final result
        // with its part.

        pub fn multi_thread() {

            // If a type implements Send, then it can be transferred between threads
            //
            // If a type implements Sync, then it can be referenced between threads.

            // Arc implements Send and Sync, whereas Rc doesn't.

            let counter = Arc::new(Mutex::new(0));
            let mut handles = vec![];

            for _ in 0..10 {
                let counter = Arc::clone(&counter);
                let handle = thread::spawn(move || {
                    let mut num = counter.lock().unwrap();

                    *num += 1;
                });
                handles.push(handle);
            }

            for handle in handles {
                handle.join().unwrap();
            }

            println!("Result: {}", *counter.lock().unwrap());
        }
    }
}
