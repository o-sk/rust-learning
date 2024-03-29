use std::sync::Arc;
use std::sync::mpsc;
use std::sync::Mutex;
use std::thread;

trait FnBox {
  fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
  fn call_box(self: Box<F>) {
    (*self)()
  }
}

pub struct ThreadPool {
  workers: Vec<Worker>,
  sender: mpsc::Sender<Job>,
}

type Job = Box<FnBox + Send + 'static>;

enum Message {
  NewJob(Job),
  Terminate,
}

impl ThreadPool {
  pub fn new(size: usize) -> ThreadPool {
    assert!(size > 0);

    let (sender, receiver) = mpsc::channel();
    let receiver = Arc::new(Mutex::new(receiver));
    let mut workers = Vec::with_capacity(size);

    for id in 0..size {
      workers.push(Worker::new(id, Arc::clone(&receiver)));
    }
    ThreadPool {
      workers,
      sender,
    }
  }

  pub fn execute<F>(&self, f: F)
    where
      F: FnOnce() + Send + 'static
  {
    let job = Box::new(f);

    self.sender.send(job).unwrap();
  }
}

impl Drop for ThreadPool {
  fn drop(&mut self) {
    for worker in &mut self.workers {
      println!("Shutting down worker {}", worker.id);

      if let Some(thread) = worker.thread.take() {
        thread.join().unwrap();
      }
    }
  }
}

struct Worker {
  id: usize,
  thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
  fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
    let thread = thread::spawn(move || {
      let job = receiver.lock().unwrap().recv().unwrap();

      println!("Worker {} got a job; executing.", id);

      job.call_box();
    });

    Worker {
      id,
      thread: Some(thread),
    }
  }
}
