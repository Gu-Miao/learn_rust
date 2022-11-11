use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
  workers: Vec<Worker>,
  sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
  /// Create a new ThreadPool
  ///
  /// The size is the number of threads in the pool.
  ///
  /// # Panics
  ///
  /// The `new` function will panic if the size is zero.
  pub fn new(size: usize) -> ThreadPool {
    assert!(size > 0);

    let mut workers = Vec::with_capacity(size);
    let (sender, receiver) = mpsc::channel();

    // 多个线程共享一个接收者
    let receiver = Arc::new(Mutex::new(receiver));

    for id in 0..size {
      workers.push(Worker::new(id, Arc::clone(&receiver)));
    }

    ThreadPool {
      workers,
      sender: Some(sender),
    }
  }

  pub fn execute<F>(&self, f: F)
  where
    F: FnOnce() + Send + 'static,
  {
    let job = Box::new(f);
    self.sender.as_ref().unwrap().send(job).unwrap();
  }
}

impl Drop for ThreadPool {
  fn drop(&mut self) {
    println!("Drop sender");
    drop(self.sender.take());

    for worker in &mut self.workers {
      println!("[Waiting for close]: {}", worker.id);
      if let Some(thread) = worker.thread.take() {
        thread.join().unwrap();
      }
      println!("[Close]: {}", worker.id);
    }
  }
}

struct Worker {
  id: usize,
  thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
  fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
    let thread = thread::spawn(move || loop {
      // 当发送者被清理后，所有的 recv 方法都会返回错误，可以通过此来判断是否需要跳出循环
      let message = receiver.lock().unwrap().recv();

      if let Ok(job) = message {
        println!("[NewJob]: {}", id);
        job();
      } else {
        println!("[Terminate]: {}", id);
        break;
      }
    });

    Worker {
      id,
      thread: Some(thread),
    }
  }
}
