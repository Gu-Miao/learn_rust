// Rust 是面向对象编程语言吗？
// Rust 收到多种编程语言范式的影响，包括面向对象
// 面向对象通常包含以下特性：命名对象、封装、继承

// 对象包含数据行为
// “设计模式四人帮”在《设计模式》中给出的面向对象定义：
// 面向对象的程序由对象组成
// 对象包装了数据和操作这些数据的过程，这些过程通常被称为方法或操作
// 基于这个定义，Rust 是面向对象的
// 结构体，枚举都包含数据
// impl 块为之提供方法
// 但带有数据的结构体和枚举并没有被称为对象

// 封装
// 调用对象外部代码无法访问对象内部的细节，唯一可以与之交互的是公开的 API
// Rust 中使用 pub 关键字

// 继承
// 是对象沿用另外一个对象的数据和行为，且无需重复定义相同的代码
// Rust 中没有继承
// Rust 中通过 trait 来进行代码共享，实现了类似继承的效果
// Rust 使用过泛型和 trait bound 的方式实现多态
// 很多新语言已经不再使用继承作为内置的程序设计方案了

// 为共有行为定义一个 trait
// Rust 避免将 Struct 或 enum 称为对象，因为它们与 impl 块是分开的
// trait 对象在某种程度上组合了数据与行为
// 但无法为 trait 对象添加数据
// trait 对象用于抽象某些共有行为，它没有其他语言中的对象那么通用

trait Draw {
  fn draw(&self);
}

struct Screen {
  // dyn Draw 是一个 trait 对象，是一种动态派发，表示 Box 中的元素都实现了 Draw trait
  components: Vec<Box<dyn Draw>>,
}

impl Screen {
  fn run(&self) {
    for component in self.components.iter() {
      component.draw();
    }
  }
}

#[allow(dead_code)]
struct Button {
  width: u32,
  height: u32,
  label: String,
}

impl Draw for Button {
  fn draw(&self) {
    println!("Button is drawing...");
  }
}

#[allow(dead_code)]
struct Select {
  width: u32,
  height: u32,
  options: Vec<String>,
}

impl Draw for Select {
  fn draw(&self) {
    println!("Select is drawing...");
  }
}

fn create_screen() {
  let screen = Screen {
    components: vec![
      Box::new(Button {
        width: 20,
        height: 12,
        label: String::from("Submit"),
      }),
      Box::new(Select {
        width: 20,
        height: 12,
        options: vec![
          String::from("Ok"),
          String::from("Maybe"),
          String::from("No"),
        ],
      }),
    ],
  };

  screen.run();
}

// 使用泛型约束时，Rust 编译器会执行单态化，确定需要调用的代码，静态派发
// 使用 trait 对象则是动态派发，需要一些运行时开销

struct Draft {
  content: String,
}

impl Draft {
  fn new() -> Draft {
    Draft {
      content: String::new(),
    }
  }
  fn add_text(&mut self, text: &str) {
    self.content.push_str(text);
  }
  fn reuest_review(self) -> PendingReview {
    PendingReview {
      content: self.content,
    }
  }
}

struct PendingReview {
  content: String,
}

impl PendingReview {
  fn approve(self) -> Post {
    Post {
      content: self.content,
    }
  }
}

struct Post {
  content: String,
}

impl Post {
  fn content(&self) -> &str {
    &self.content
  }
}

// Rust 可以实现面向对象设计模式
// 但 Rust 具有所有权等其他面向对象编程语言所没有的特性，因此面向对象设计模式
// 并不总是 Rust 编程中的最佳选择
fn rust_style_oop() {
  let mut post = Draft::new();
  post.add_text("Some text of post");
  let post = post.reuest_review();
  let post = post.approve();

  println!("content of post: {}", post.content());
}

fn main() {
  create_screen();
  rust_style_oop();
}
