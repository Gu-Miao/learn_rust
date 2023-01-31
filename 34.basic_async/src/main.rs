use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::thread::{self, sleep};
use std::time::{Duration, Instant};

// 异步编程就是在同一线程上，同时运行多个任务，就是一个任务等待时，去执行另一个任务
// async 和 await 本质上是一种语法糖

async fn f1() -> String {
  sleep(Duration::new(4, 0));
  println!("1");
  String::from("1")
}

async fn f2() -> String {
  sleep(Duration::new(2, 0));
  println!("2");
  String::from("2")
}

// 简单的异步任务
async fn simple_async_tasks() {
  println!("===simple_async_tasks===");

  let h1 = tokio::spawn(async {
    // async 函数是惰性的，只有后面 .await 才会执行
    f1().await;
  });

  let h2 = tokio::spawn(async {
    f2().await;
  });

  let _ = tokio::join!(h1, h2);
}

// Rust 的异步核心是 Future
// Rust 的异步函数都会返回 Future trait，他代表延迟的计算（类似于 JavaScript 中的 Promise）
// Future trait 中有一个 poll 方法，主要供异步执行器调用，用来确定当前的异步任务是否完成
// poll 方法返回 Poll::Pending 或 Poll::Ready(val) 枚举，表示进行中和已完成
// _f1 函数和 f1 函数是等价的
fn _f1() -> impl Future<Output = String> {
  async {
    sleep(Duration::new(4, 0));
    println!("1");
    String::from("1")
  }
}

struct ReadFileFuture {}

impl Future for ReadFileFuture {
  type Output = String;
  fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
    println!("ReadFileFuture poll");

    sleep(Duration::new(2, 0));

    // Rust 不会一直调用 poll 方法而是使用一个 Waker 组件来确定任务是否完成
    // 异步执行器调用过 poll 方法但没完成的异步任务会被注册到 Waker 上。
    // Waker 会有一个处理程序（handle），它会被存储在关联的 Context 对象上。
    // Wakder 有一个 wake 方法，可以用来告诉异步执行器关联的任务被唤醒了。这时异步执行器会再次调用
    // poll 函数
    cx.waker().wake_by_ref();
    Poll::Ready(String::from("Poll ready"))
  }
}

async fn custom_future() {
  println!("\n===custom_future===");

  let h1 = tokio::spawn(async {
    println!("{}", ReadFileFuture {}.await);
  });
  let h2 = tokio::spawn(async {
    f1().await;
  });
  let _ = tokio::join!(h1, h2);
}

struct AsyncTimer {
  expiration_time: Instant,
}

impl Future for AsyncTimer {
  type Output = String;

  fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
    println!("AsyncTimer poll");

    if Instant::now() >= self.expiration_time {
      Poll::Ready(String::from("AsyncTimer ready"))
    } else {
      println!("Not yet, go sleep");

      let waker = cx.waker().clone();
      let expiration_time = self.expiration_time;
      thread::spawn(move || {
        let current_time = Instant::now();
        if current_time < expiration_time {
          thread::sleep(expiration_time - current_time);
        }
        waker.wake();
      });
      Poll::Pending
    }
  }
}

async fn timer() {
  println!("\n===timer===");

  let h1 = tokio::spawn(async {
    let timer = AsyncTimer {
      expiration_time: Instant::now() + Duration::from_millis(4000),
    };
    println!("{}", timer.await);
  });
  let h2 = tokio::spawn(async {
    f1().await;
  });
  let _ = tokio::join!(h1, h2);
}

#[tokio::main]
async fn main() {
  simple_async_tasks().await;
  custom_future().await;
  timer().await;
}
