mod PoolCreationErrorModule;
mod Worker;
//use crate::Worker::Worker;
use crate::PoolCreationErrorModule::PoolCreationError;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;


pub struct ThreadPool {
    workers: Vec<Worker::Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size <= 0 {
            Err(PoolCreationError)
        }
        else {
            let (sender, receiver) = mpsc::channel();
            let receiver = Arc::new(Mutex::new(receiver));

            let mut workers = Vec::with_capacity(size);
            for id in 0..size {
                workers.push(Worker::Worker::new( id, Arc::clone(&receiver) ));
            }

            Ok( ThreadPool {workers, sender} )
        }
    }

    pub fn execute<F>(&self, f: F)
    where 
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
