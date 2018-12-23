use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct ThreadPool {
  workers: Vec<Worker>,
  sender: mpsc::Sender<Job>,
}

impl ThreadPool {
  pub fn execute<F>(&self, f: F)
  where
    F: FnOnce() + Send + 'static,
  {
    let job = Box::new(f);
    self.sender.send(job).unwrap();
  }
  pub fn new(size: usize) -> ThreadPool {
    assert!(size > 0); // validation that required poll size contains more than 0 threads

    let (sender, receiver) = mpsc::channel();

    let receiver = Arc::new(Mutex::new(receiver));

    let mut workers = Vec::with_capacity(size);

    for id in 0..size {
      workers.push(Worker::new(id, Arc::clone(&receiver)));
    }
    ThreadPool { workers, sender }
  }
  // pub fn execute<F>(&self, _f: F)
  // where
  //   F: FnOnce() + Send + 'static,
  // {

  // }
}

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

type Job = Box<dyn FnBox + Send + 'static>;

struct Worker {
  id: usize,
  thread: thread::JoinHandle<()>,
}

impl Worker {
  fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
    let thread = thread::spawn(move || loop {
      let job = receiver.lock().unwrap().recv().unwrap();

      println!("Worker {} got a job; executing.", id);

      job.call_box();
    });

    Worker { id, thread }
  }
}
// trait FnBox {
//   fn call_box(self: Box<Self>);
// }

// impl<F: FnOnce()> FnBox for F {
//   fn call_box(self: Box<F>) {
//     (*self)()
//   }
// }
