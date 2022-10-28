// Rust 号称无畏并发，允许你编写没有细微 bug 的代码，并在不引入新 bug 的情况下易于重构

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

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

// Go 的名言：不要用共享内存通信，要用通信来共享内存
fn message_pass() {
  let (tx, rx) = mpsc::channel();

  thread::spawn(move || {
    let s = String::from("hello");

    // s 的所有权转移
    tx.send(s).unwrap();

    // s 已经失效
    // println!("{}", s);
  });

  // recv 方法会阻塞当前线程，直到有消息传入或者所有发送端关闭
  let message = rx.recv().unwrap();
  println!("{}", message);
}

fn send_many() {
  let (tx, rx) = mpsc::channel();

  thread::spawn(move || {
    let v = vec![
      String::from("Hello"),
      String::from("I'm GuMiao"),
      String::from("I come from China"),
      String::from("Nice to meet you"),
    ];

    for s in v {
      tx.send(s).unwrap();
      thread::sleep(Duration::from_millis(300));
    }
  });

  // 可以将接收端当作一个迭代器使用
  for message in rx {
    println!("rx: {}", message);
  }
}

fn many_sender() {
  let (tx, rx) = mpsc::channel();
  let tx1 = mpsc::Sender::clone(&tx);

  thread::spawn(move || {
    let v = vec![
      String::from("1: Welcome"),
      String::from("1: Welcome"),
      String::from("1: Happy Hunger game"),
      String::from("1: Ha Ha Ha"),
    ];

    for s in v {
      tx1.send(s).unwrap();
      thread::sleep(Duration::from_millis(300));
    }
  });

  thread::spawn(move || {
    let v = vec![
      String::from("Hello"),
      String::from("I'm GuMiao"),
      String::from("I come from China"),
      String::from("Nice to meet you"),
    ];

    for s in v {
      tx.send(s).unwrap();
      thread::sleep(Duration::from_millis(300));
    }
  });

  // 可以将接收端当作一个迭代器使用
  for message in rx {
    println!("rx: {}", message);
  }
}

fn main() {
  hello_thread();

  message_pass();

  send_many();

  many_sender();
}
