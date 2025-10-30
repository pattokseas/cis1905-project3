use std::{
    mem,
    sync::{mpsc, Arc, Mutex},
    thread,
};

// We represent a job as a boxed closure that can be sent across threads. Since the closure is
// `Send`, it can be sent across threads. Since it is in a box, we have ownership and can transfer
// it to other threads.
type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}
impl Worker {
    // Spawn a new thread that will loop forever, receiving jobs from the receiver and executing
    // them. If the `recv()` method returns an error, it means the thread pool has been dropped and
    // the thread should exit by breaking the loop.
    // This function should return a `Worker` as a handle to the thread.
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let handle = thread::spawn(move || loop {
            let channel = receiver.lock().unwrap();
            match channel.recv() {
                Ok(f) => {
                    drop(channel);
                    f();
                }
                Err(_) => {
                    return;
                }
            };
        });
        Worker {
            id: id,
            thread: Some(handle),
        }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    // Spawn `size` workers by calling the `Worker::new` function `size` times, each time with a
    // unique id. You will need to create a channel and wrap the receiver in an `Arc<Mutex<...>>`
    // in order to share it with the worker threads. Finally, return an instance of `ThreadPool`
    // that has the workers and the sender.
    pub fn new(size: usize) -> ThreadPool {
        let (tx, rx) = mpsc::channel();
        let mut workers = Vec::new();
        let receiver = Arc::new(Mutex::new(rx));
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool {
            workers: workers,
            sender: Some(tx),
        }
    }

    // Send the job `f` to the worker threads via the channel `send` method.
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.as_ref().unwrap().send(Box::new(f)).unwrap();
    }
}

impl Drop for ThreadPool {
    // First, take ownership of the sender from inside the option, then drop it. This will trigger
    // the worker threads to stop since the channel is closed, so you should then call `join` on
    // each worker thread handle to make sure they finish executing. Calling `join` will also
    // require you to take ownership of the worker thread handle from inside the option.
    fn drop(&mut self) {
        let sender = mem::take(&mut self.sender).unwrap();
        drop(sender);
        for worker in self.workers.iter_mut() {
            mem::take(&mut worker.thread).unwrap().join().unwrap();
        }
    }
}
