pub mod PoolCreationErrorModule {
    pub struct PoolCreationError;
}

use std::{error::Error, fmt};

//#[derive(Debug)]
pub struct PoolCreationError;

impl Error for PoolCreationError {}

impl fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error creating a thread pool: thread size cannot be 0!")
    }
}

impl std::fmt::Debug for PoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error creating a thread pool: thread size cannot be 0! file: {}, line: {}",
                file!(), line!())
    }
}