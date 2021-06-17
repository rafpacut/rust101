mod PoolCreationErrorModule;
mod Worker;
//use crate::Worker::Worker;
use crate::PoolCreationErrorModule::PoolCreationError;

pub struct ThreadPool {
    workers: Vec<Worker::Worker>,
}


impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.

    //TODO
    //create another 'new'
    pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size <= 0 {
            Err(PoolCreationError)
        }
        else {
            let mut workers = Vec::with_capacity(size);
            for id in 0..size {
                workers.push(Worker::Worker::new( id ));
            }

            Ok( ThreadPool {workers} )
        }
    }

    pub fn execute<F>(&self, f: F)
    where 
        F: FnOnce() + Send + 'static,
    {

    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
