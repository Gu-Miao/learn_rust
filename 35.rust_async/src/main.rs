// Rust 中的异步与多线程
// OS 线程：
// 1. 适用于少量任务，有内存与 CPU 开销，且线程生成的线程间切换非常昂贵
// 2. 线程池可以降低一些成本
// 3. 允许重用同步代码，代码无需大改，无需特定编程模型
// 4. 有些系统支持修改线程优先级
// 异步：
// 1. 显著降低内存和 CPU 开销
// 2. 同等条件下，支持比线程多几个数量级的任务（少数线程支撑大量任务）
// 3. 可执行文件大（需要生成状态机，每个可执行文件捆绑一个异步运行时）

// 标准库提供了最基本的特性类型和功能，比如 Future trait
// Rust 编译器直接支持 async/await 语法
// future crate 提供了许多适用类型、宏和函数
// 异步代码、IO 和任务生成的执行依赖于异步运行时，大多来自社区。比如 tokio 和 async-std

// Rust 不允许在 trait 中声明 async 函数（async-trait）

// 异步代码和同步代码不能总是自由组合，比如不能直接从同步函数来调用
// 异步代码间也不总是能自由组合，一些 crate 依赖于特定的异步运行时
// 所以，尽早确定需要使用的异步运行时
// 同时异步代码的性能也依赖于异步运行时（通常性能都很高）

use futures::executor::block_on;
use std::task::Poll;

// 异步函数返回的是一个实现了 Future trait 的类型
// Future 是惰性的，需要一个执行者来运行
async fn hello_world() {
  println!("async hello world");
}

fn run_future() {
  let future = hello_world(); // 这里不会打印
  block_on(future); // 使用 block_on 执行，它会阻塞当前线程直到 Future 完成
}

// 也可以使用 .await 来等待一个 Future 的完成，使用 .await 不会阻塞当前线程

// Future trait 是 Rust 异步编程的核心
// 它本质上是一种异步计算，可以产生一个值
// 实现了 Fture trait 的类型表示目前可能还不可用的值

// 一个简化版的 Future trait
trait SimpleFuture {
  // 占位类型，代表最终 Future 输出的类型
  type Output;
  // 调用 poll 方法会驱动 Future 向着完成的方向前进
  // Poll 枚举来自 std::tasks，它有两个变体 Pending 和 Ready<T>
  // Pending 表示未完成 Ready<T> 表示完成
  // 当 Future 准备取得更多进展时可以调用 wake 上的方法告诉异步执行器，再次调用 poll 方法
  // 对于 Future，你唯一能做的就是一直调用 poll 方法来推动它，直到产生一个可用的值
  fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
}

fn main() {
  run_future()
}
