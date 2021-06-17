use std::thread;

//pub mod Worker {
    //pub struct Worker;
//}

pub struct Worker {
    id: usize,
    handle: thread::JoinHandle<()>,
}

impl Worker {
    pub fn new(id: usize) -> Worker {
        Worker { id: id, handle: thread::spawn(|| {}) }
    }
}