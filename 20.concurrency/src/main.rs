// Rust 号称无畏并发，允许你编写没有细微 bug 的代码，并在不引入新 bug 的情况下易于重构

use std::thread;

fn hello_thread() {
  let v = 10;

  // 使用 move 关键字获取 v 的所有权
  // 但 v 是基础类型，这里只发生了复制，主线程中的 v 仍然有效
  let handle = thread::spawn(move || {
    for i in 0..v {
      println!("i from spawn: {}", i);
    }
  });

  handle.join().unwrap();

  for i in 0..(v / 2) {
    println!("i from main: {}", i);
  }
}

fn main() {
  hello_thread();
}
