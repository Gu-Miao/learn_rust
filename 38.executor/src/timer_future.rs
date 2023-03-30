use std::{
  future::Future,
  pin::Pin,
  sync::{Arc, Mutex},
  task::{Context, Poll, Waker},
  thread,
  time::Duration,
};

pub struct TimerFuture {
  shared_state: Arc<Mutex<SharedState>>,
}

struct SharedState {
  completed: bool,
  waker: Option<Waker>,
}

impl Future for TimerFuture {
  type Output = ();

  fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
    println!("[{:?}] Polling TimerFuture...", thread::current().id());

    let mut shared_state = self.shared_state.lock().unwrap();

    if shared_state.completed {
      println!("[{:?}] TimeFuture ready...", thread::current().id());

      Poll::Ready(())
    } else {
      println!("[{:?}] TimeFuture pending...", thread::current().id());

      shared_state.waker = Some(cx.waker().clone());
      Poll::Pending
    }
  }
}

impl TimerFuture {
  pub fn new(duration: Duration) -> Self {
    println!("[{:?}] Creating new TimerFuture...", thread::current().id());

    let shared_state = Arc::new(Mutex::new(SharedState {
      completed: false,
      waker: None,
    }));

    let thread_shared_state = shared_state.clone();
    thread::spawn(move || {
      println!(
        "[{:?}] Creating new thread and start sleeping...",
        thread::current().id()
      );

      thread::sleep(duration);
      let mut shared_state = thread_shared_state.lock().unwrap();

      shared_state.completed = true;
      if let Some(waker) = shared_state.waker.take() {
        println!(
          "[{:?}] Getting waker and run wake()...",
          thread::current().id()
        );

        waker.wake()
      } else {
        println!("[{:?}] No waker got...", thread::current().id());
      }
    });

    println!(
      "[{:?}] Returning new TimerFuture...",
      thread::current().id()
    );

    TimerFuture { shared_state }
  }
}
