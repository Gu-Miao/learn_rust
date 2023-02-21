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

// 异步函数返回的是一个实现了 Future trait 的状态机
// Future 是惰性的，需要一个执行者来运行
async fn hello_world() {
  println!("async hello world");
}

fn run_future() {
  let future = hello_world(); // 这里不会打印
  block_on(future); // 使用 block_on 执行，它会阻塞当前线程直到 Future 完成
}

// 也可以使用 .await 来等待一个 Future 的完成，使用 .await 不会阻塞当前线程，而是异步地等待其完成

struct Song {}

async fn learn_song() -> Song {
  Song {}
}

async fn sing(_: Song) {}
async fn dance() {}

async fn learn_and_sing() {
  let song = learn_song().await;
  sing(song).await;
}

// 异步地唱歌和跳舞
async fn sing_and_dance() {
  let f1 = learn_and_sing();
  let f2 = dance();
  futures::join!(f1, f2);
}

fn main() {
  run_future();
  block_on(sing_and_dance());
}
