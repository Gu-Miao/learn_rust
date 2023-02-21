// Future 在第一次 poll 时通常无法完成任务，所以 Future 需要保证
// 每次 Future 被 poll，他都是作为一个任务的一部分
// 任务就是被提交给执行器的顶层 Future
// Waker 实现了 Clone trait，可以复制和存储

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

/// 在 Future 和等待的线程间共享状态
struct SharedState {
  /// 睡眠时间是否已经都过完
  completed: bool,
  /// TimerFuture 所运行于的任务的 Waker
  /// 当 completed 为 ture 之后，线程可以使用它来告诉
  /// TimerFuture 的任务可以唤醒，看到 completed = true 并前进
  waker: Option<Waker>,
}

impl Future for TimerFuture {
  type Output = ();

  fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
    let mut shared_state = self.shared_state.lock().unwrap();

    // 查看 shared_state，看下 timer 是否已经结束
    if shared_state.completed {
      Poll::Ready(())
    } else {
      // 设置 waker 以便当 timer 结束时线程可以唤醒当前任务，保证
      // Future 可以再次被 poll，并看到 completed = true
      //
      // 相比每次都克隆 waker，如果只做一次显然更有诱惑力
      // 但是 TimerFuture 可以在执行器的任务间移动，这会导致
      // 过期的 waker 执行错误的任务，从而阻止 TimerFuture
      // 正确的唤醒
      //
      // 注意：可以使用 Waker::will_wake 函数来检查这一点
      // 但为了简单起见，我们就省略了这一点
      shared_state.waker = Some(cx.waker().clone());
      Poll::Pending
    }
  }
}

impl TimerFuture {
  /// 创建一个新的 TimerFuture，它将在提供的时限过后完成
  pub fn new(duration: Duration) -> Self {
    let shared_state = Arc::new(Mutex::new(SharedState {
      completed: false,
      waker: None,
    }));

    // 生成新线程
    let thread_shared_state = shared_state.clone();
    thread::spawn(move || {
      thread::sleep(duration);
      let mut shared_state = thread_shared_state.lock().unwrap();

      // 发出信号：计时器已经停止并唤醒 Future 被 poll 的最后一个任务
      // （如果存在的话）
      shared_state.completed = true;
      if let Some(waker) = shared_state.waker.take() {
        waker.wake()
      }
    });

    TimerFuture { shared_state }
  }
}
