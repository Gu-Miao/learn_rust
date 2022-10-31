// Rust 号称无畏并发，允许你编写没有细微 bug 的代码，并在不引入新 bug 的情况下易于重构

use std::sync::{mpsc, Arc, Mutex};
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

// 共享状态的并发，即 Go 语言中不推荐的方式
// 共享内存类似多所有权，多个线程可以同时访问同一块内存
// Mutex 是互斥锁 mutual exclusion 的简写
// 同一时刻，Mutex 只允许一个线程来访问某些数据
// 想要访问数据，线程必须先获取互斥锁（lock），它是 Mutex 一部分，它可以跟踪谁对数据持有独占访问权
// Mutex 通常被描述为通过锁定系统来保护他持有的数据
// 使用 Mutex 的两条规则：
// 1. 在使用数据前，必须尝试获得锁
// 2. 使用完数据，需要对其进行解锁，以便其他线程可以获取锁
fn hello_mutex() {
  // Mutex 是一个智能指针
  let m = Mutex::new(5);

  {
    // lock 方法会返回一个 MutexGuard 类型，它是一个智能指针
    let mut num = m.lock().unwrap();
    *num += 5;
  } // 因为 MutexGuard 实现了 Drop trait，它失效时会自动解锁

  println!("m is {:?}", m);
}

// 线程间传递数据不能用 Rc<T>，它只适用于单线程场景
// 只有实现了 Send trait 的类型才能够安全地在线程间传递
// Arc<T> 需要牺牲性能为代价，因此标准库默认没有使用它
// Arc<T> 与 Rc<T> 的 API 一样
fn mutex_thread() {
  let counter = Arc::new(Mutex::new(0));
  let mut handles = vec![];

  for _ in 0..10 {
    let counter = Arc::clone(&counter);
    let handle = thread::spawn(move || {
      let mut number = counter.lock().unwrap();
      *number += 1;
    });
    handles.push(handle);
  }

  for handle in handles {
    handle.join().unwrap();
  }

  println!("counter is {}", *counter.lock().unwrap());
}

// RefCell<T>/Rc<T> vs Mutex<T>/Arc<T>
// Mutex<T> 提供了内部可变性，与 Cell 家族一样
// RefCell<T> 可以改变 Rc<T> 中的内容，Mutex<T> 可以改变 Arc<T> 中的内容
// RefCell<T> 有循环引用的风险，Mutex<T> 有死锁的风险

// Send trait 允许在线程间转移所有权，几乎所有类型都实现了 Send trait，但 Rc<T> 没有
// 任何完全由实现了 Send trait 的类型组成的类型（结构体，元组等）也被标为实现了 Send trait
// 除了原始指针外，几乎所有基础类型都实现了 Send trait

// 实现 Sync trait 的类型可以安全的被多个线程引用
// 如果 T 实现了 Sync trait，那么 &T 就被视为实现了 Send trait，即它的引用可以被送往另一个线程
// 基础类型都实现了 Sync trait
// 完全由实现了 Sync trait 的类型组成的类型也被视为实现了 Sync trait
// Rc<T>, RefCell<T> 和 Cell 家族未实现 Sync trait
//

fn main() {
  hello_thread();
  message_pass();
  send_many();
  many_sender();
  hello_mutex();
  mutex_thread();
}
