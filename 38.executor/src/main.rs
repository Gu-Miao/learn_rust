// Future 是惰性的：除非驱动他们来完成，否则就什么都不做
// 一种驱动方式是在 async 函数里使用 .await，但这只是把问题推到了上一层面，所以需要一个执行者
// Future 执行者会获取一系列顶层的 Future，通过在 Future 可以有进展的时候调用 poll，来将这些
// Future 运行至完成
// 通常首先，执行者将 poll 一个 Future 一次来开始
// 当 Future 通过调用 wake 方法表示他们已经准备好取得进展时，他们就会比放到一个队列里，然后 poll
// 再次被调用，重复此操作直到 Future 完成

// 构建简单的执行者，可以运行大量的顶层 Future 来并发地完成
// 需要使用 futures crate 的 ArcWake trait
mod timer_future;

use futures::{
  future::{BoxFuture, FutureExt},
  task::{waker_ref, ArcWake},
};
use std::{
  future::Future,
  sync::mpsc::{sync_channel, Receiver, SyncSender},
  sync::{Arc, Mutex},
  task::Context,
  time::Duration,
};
use timer_future::TimerFuture;

/// Task executor that receives tasks off of a channel and runs them.
struct Executor {
  ready_queue: Receiver<Arc<Task>>,
}

impl Executor {
  fn run(&self) {
    while let Ok(task) = self.ready_queue.recv() {
      // Take the future, and if it has not yet completed (is still Some),
      // poll it in an attempt to complete it.
      let mut future_slot = task.future.lock().unwrap();
      if let Some(mut future) = future_slot.take() {
        // Create a `LocalWaker` from the task itself
        let waker = waker_ref(&task);
        let context = &mut Context::from_waker(&*waker);
        // `BoxFuture<T>` is a type alias for
        // `Pin<Box<dyn Future<Output = T> + Send + 'static>>`.
        // We can get a `Pin<&mut dyn Future + Send + 'static>`
        // from it by calling the `Pin::as_mut` method.
        if future.as_mut().poll(context).is_pending() {
          // We're not done processing the future, so put it
          // back in its task to be run again in the future.
          *future_slot = Some(future);
        }
      }
    }
  }
}

/// `Spawner` spawns new futures onto the task channel.
#[derive(Clone)]
struct Spawner {
  task_sender: SyncSender<Arc<Task>>,
}

impl Spawner {
  fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
    let future = future.boxed();
    let task = Arc::new(Task {
      future: Mutex::new(Some(future)),
      task_sender: self.task_sender.clone(),
    });
    self.task_sender.send(task).expect("too many tasks queued");
  }
}

/// A future that can reschedule itself to be polled by an `Executor`.
struct Task {
  /// In-progress future that should be pushed to completion.
  ///
  /// The `Mutex` is not necessary for correctness, since we only have
  /// one thread executing tasks at once. However, Rust isn't smart
  /// enough to know that `future` is only mutated from one thread,
  /// so we need to use the `Mutex` to prove thread-safety. A production
  /// executor would not need this, and could use `UnsafeCell` instead.
  future: Mutex<Option<BoxFuture<'static, ()>>>,

  /// Handle to place the task itself back onto the task queue.
  task_sender: SyncSender<Arc<Task>>,
}

impl ArcWake for Task {
  fn wake_by_ref(arc_self: &Arc<Self>) {
    // Implement `wake` by sending this task back onto the task channel
    // so that it will be polled again by the executor.
    let cloned = arc_self.clone();
    arc_self
      .task_sender
      .send(cloned)
      .expect("too many tasks queued");
  }
}

fn new_executor_and_spawner() -> (Executor, Spawner) {
  // Maximum number of tasks to allow queueing in the channel at once.
  // This is just to make `sync_channel` happy, and wouldn't be present in
  // a real executor.
  const MAX_QUEUED_TASKS: usize = 10_000;
  let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);
  (Executor { ready_queue }, Spawner { task_sender })
}

fn example() {
  let (executor, spawner) = new_executor_and_spawner();

  // Spawn a task to print before and after waiting on a timer.
  spawner.spawn(async {
    println!("howdy!");
    // Wait for our timer future to complete after two seconds.
    TimerFuture::new(Duration::new(2, 0)).await;
    println!("done!");
  });

  // Drop the spawner so that our executor knows it is finished and won't
  // receive more incoming tasks to run.
  drop(spawner);

  // Run the executor until the task queue is empty.
  // This will print "howdy!", pause, and then print "done!".
  executor.run();
}

fn main() {
  example();
}
