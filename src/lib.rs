use std::error::Error;
use std::thread;
use std::ops::Add;
use std::os::raw::c_void;

pub struct ThreadPool{
    workers: Vec<Worker>
}

pub struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>
}
impl Worker{
    pub fn new(id: usize) -> Worker {
        let thread = thread::spawn(||{});
        Worker{ id, thread }
    }
}

impl ThreadPool {

    pub fn new(size: usize)-> ThreadPool {
        assert!(size>0);
        let mut workers = Vec::with_capacity(size);
        for i in 0..size {
            let wrkr = Worker::new(i);
            workers.push(wrkr);
        }
        ThreadPool{workers:workers}
    }

    pub fn execute <F> (&self, f:F)
        where
            F: FnOnce() + Send + 'static
    {
    }
}

