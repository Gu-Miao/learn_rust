// Future trait 是 Rust 异步编程的核心
// 它本质上是一种异步计算，可以产生一个值
// 实现了 Fture trait 的类型表示目前可能还不可用的值

use std::{
  pin::Pin,
  task::{Context, Poll},
};

// 一个简化版的 Future trait
trait SimpleFuture {
  // 占位类型，代表最终 Future 输出的类型
  type Output;
  // 调用 poll 方法会驱动 Future 向着完成的方向前进
  // Poll 枚举来自 std::tasks，它有两个变体 Pending 和 Ready<T>
  // Pending 表示未完成 Ready<T> 表示完成
  // 当 Future 准备取得更多进展时可以调用 wake 上的方法告诉异步执行器，再次调用 poll 方法以推动 Future 获得更多进展
  // 对于 Future，你唯一能做的就是一直调用 poll 方法来推动它，直到产生一个可用的值或错误
  fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
}

// Socket 类的伪实现
struct Socket {}

impl Socket {
  fn has_data_to_read(&self) -> bool {
    true
  }
  fn read_buf(&self) -> Vec<u8> {
    vec![]
  }
  fn set_readable_callback(&self, _: fn()) {}
}

pub struct SocketRead<'a> {
  socket: &'a Socket,
}

impl SimpleFuture for SocketRead<'_> {
  type Output = Vec<u8>;

  fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
    if self.socket.has_data_to_read() {
      // 有数据，直接读取然后返回
      Poll::Ready(self.socket.read_buf())
    } else {
      // 没有数据，那就等待数据，并且返回 Pending
      // set_readable_callback 接收一个函数指针，当 soket 有可读取的数据时，执行回调
      self.socket.set_readable_callback(wake);
      Poll::Pending
    }
  }
}

/// 组合多个异步操作，无需中间分配
/// 可以通过无分配的状态机来实现多个 Future 同时运行或串联运行
pub struct Join<FutureA, FutureB> {
  // 每个字段可能包含一个应该运行到完成的 Future。
  // 如果 Future 已经完成，则该字段将被设为 None。
  // 这阻止了我们在 Future 完成后再次调用其 poll 方法，这个违反了 Future 性质的行为。
  a: Option<FutureA>,
  b: Option<FutureB>,
}

impl<FutureA, FutureB> SimpleFuture for Join<FutureA, FutureB>
where
  FutureA: SimpleFuture<Output = ()>,
  FutureB: SimpleFuture<Output = ()>,
{
  type Output = ();
  fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
    // 尝试去完成 Future A，如果 A 完成那么将 `a` 字段设为 None
    if let Some(a) = &mut self.a {
      if let Poll::Ready(()) = a.poll(wake) {
        self.a.take();
      }
    }

    // 同上
    if let Some(b) = &mut self.b {
      if let Poll::Ready(()) = b.poll(wake) {
        self.b.take();
      }
    }

    if self.a.is_none() && self.b.is_none() {
      Poll::Ready(())
    } else {
      Poll::Pending
    }
  }
}

/// 一个简单的 Future，运行两个 Future，让它们一个接一个地完成。
//
// 注意：在这个简单示例中，AndThenOut 假设第一个 Future 和第二个 Future
// 在创建时都可用。真正的 AndThen 组合符允许基于第一个 Future 的输出创建第
// 二个 Future，如 get_breakfast.and_then(|food| eat(food))。
pub struct AndThenFut<FutureA, FutureB> {
  first: Option<FutureA>,
  second: FutureB,
}

impl<FutureA, FutureB> SimpleFuture for AndThenFut<FutureA, FutureB>
where
  FutureA: SimpleFuture<Output = ()>,
  FutureB: SimpleFuture<Output = ()>,
{
  type Output = ();
  fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
    if let Some(first) = &mut self.first {
      match first.poll(wake) {
        // 第一个完成，将它设为 None
        Poll::Ready(()) => self.first.take(),
        // 第一个未完成
        Poll::Pending => return Poll::Pending,
      };
    }
    // 第一个已经完成，尝试完成第二个
    self.second.poll(wake)
  }
}

// 真正的 Future trait 定义
trait Future {
  type Output;
  // self 类型不再是 &mut self，而是 Pin<&mut Self>，它允许我们创建
  // 不可移动的 Future，不可移动的对象可以在它们的字段间存储指针
  // 需要启用 async/await，Pin 就是必须的
  // 第二个参数从 wake: fn() 变为 cx: &mut Context<'_>
  // 在 SimpleFuture 中 wake 是一个函数指针，它缺少存储上下文数据的能力
  // 而 Context 类型则提供了访问 Waker 类型的值的方式，这些值可以用以唤醒特定的任务
  // 比如 Web 服务器可能有上千个连接，这些值可以用于区分不同的任务
  fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

fn main() {}
