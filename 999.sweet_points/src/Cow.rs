// Rust 标准库中的 std::borrow::Cow 是一个用于获取 Cow 类型的引用的工具类型。
// 它允许你在不暴露原始 Cow 对象的情况下获取其引用。这在需要在多个线程或进程之间共
// 享资源的场景下非常有用，因为在 Rust 中，可以使用内存安全的方式来共享资源，而不需
// 要担心资源泄漏或竞争条件。

// 下面是一个使用 std::borrow::Cow 的示例：
use std::borrow::Cow;

fn main() {
  let mut c = Cow::new("Hello".to_string());
  let ptr = c.clone();

  // 使用 Cow 对象
  println!("{}", c.borrow());

  // 使用引用
  println!("{}", c.as_mut().unwrap().borrow());
}

// 在上面的示例中，我们首先使用 Cow::new 创建了一个 Cow 对象 c，然后使用 c.clone()
// 获取了其引用。接下来，我们使用 c.borrow() 获取了其 Cow 对象的引用。我们可以在同一
// 时间内获取和使用 Cow 对象的引用，这是使用 std::borrow::Cow 的关键优势。

// 使用 std::borrow::Cow 的场景非常广泛，例如在并发编程中，你可以使用它来避免资源竞争
// 和死锁。你可以在多个线程或进程之间共享一个 Cow 对象的引用，这样可以确保在多个线程或进
// 程之间安全地访问共享资源。
