use std::sync::mpsc;
use std::thread;

pub struct ThreadPool {
  workers: Vec<Worker>,
  sender: mpsc::Sender<Job>,
}

struct Job;

impl ThreadPool {
  pub fn new(size: usize) -> ThreadPool {
    assert!(size > 0); // validation that required poll size contains more than 0 threads

    let (sender, receiver) = mpsc::channel();
    let mut workers = Vec::with_capacity(size);

    for id in 0..size {
      workers.push(Worker::new(id));
    }
    ThreadPool { workers, sender }
  }
  // pub fn execute<F>(&self, _f: F)
  // where
  //   F: FnOnce() + Send + 'static,
  // {

  // }
}

struct Worker {
  id: usize,
  thread: thread::JoinHandle<()>,
}

impl Worker {
  fn new(id: usize) -> Worker {
    let thread = thread::spawn(|| {});
    Worker { id, thread }
  }
}
