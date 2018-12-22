use std::thread;

pub struct ThreadPool {
  workers: Vec<Worker>,
}

impl ThreadPool {
  pub fn new(size: usize) -> ThreadPool {
    assert!(size > 0); // validation that required poll size contains more than 0 threads
    let mut workers = Vec::with_capacity(size);

    for id in 0..size {
      workers.push(Worker::new(id));
    }

    ThreadPool { workers }
  }
  pub fn execute<F>(&self, _f: F)
  where
    F: FnOnce() + Send + 'static,
  {

  }
}
