use std::fmt::Display;
use std::fmt::{Formatter, Result};

// 声明 trait
trait Summary {
  // 默认实现
  fn summarize(&self) -> String {
    String::from("(Read more...)")
  }
}

struct News {
  headline: String,
  location: String,
  author: String,
  content: String,
}

impl Summary for News {
  // 重写实现
  fn summarize(&self) -> String {
    format!("{}, by {} ({})", self.headline, self.author, self.location)
  }
}

impl Display for News {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(
      f,
      "News {{ headline: \"{}\", location: \"{}\", author: \"{}\", content: \"{}\" }}",
      self.headline, self.location, self.author, self.content
    )
  }
}

fn main() {
  let news = News {
    headline: "十字路口发生连环事故，多人轻伤".to_string(),
    location: "河北省石家庄市中华南大街".to_string(),
    author: "王平".to_string(),
    content: "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx".to_string(),
  };
  println!("news: {}", news);
  println!("summary of new is {}", news.summarize());
  println!("content of new is {}", news.content);
}

// 实现 trait 的约束
// 类型或者 trait 是在本地 crate 中定义的
// 无法为外部类型实现外部的 trait，即孤儿原则，为了代码的一致性，可以保护外部代码不被破坏

// 想限制函数参数必须实现某个 trait
// 限制多个 trait 需要使用 + 连接
fn _fn1(item1: impl Summary, item2: impl Summary + Display) {
  println!("{}", item1.summarize());
  println!("{}", item2.summarize());
}

// trait bound 写法
// 上面 impl 写法是 trait bound 的语法糖
fn _fn2<T: Summary + Display>(item1: T, item2: T) {
  println!("{}", item1.summarize());
  println!("{}", item2.summarize());
}

// 使用 where 子句
fn _fn3<T, U>(item1: T, item2: U)
where
  T: Summary + Display,
  U: Summary + Clone,
{
  println!("{}", item1.summarize());
  println!("{}", item2.summarize());
}

// 使用 impl trait 作为返回类型
fn _fn4() -> impl Summary {
  News {
    headline: "十字路口发生连环事故，多人轻伤".to_string(),
    location: "河北省石家庄市中华南大街".to_string(),
    author: "王平".to_string(),
    content: "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx".to_string(),
  }
}

// 但 impl trait 作为返回类型必须返回同一种类型
struct Tweet {
  username: String,
  content: String,
  time: String,
}

impl Summary for Tweet {
  fn summarize(&self) -> String {
    format!("{}, by {}, at {}", self.content, self.username, self.time)
  }
}

// fn _fn5(flag: bool) -> impl Summary {
//   if flag {
//     News {
//       headline: "十字路口发生连环事故，多人轻伤".to_string(),
//       location: "河北省石家庄市中华南大街".to_string(),
//       author: "王平".to_string(),
//       content: "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx".to_string(),
//     }
//   } else {
//     // News 和 Tweet 虽然都实现了 Summary trait，但是函数能返回具体类型是唯一的，
//     // 因此这里会报错
//     Tweet {
//       username: "Sam".to_string(),
//       content: "I love Rust".to_string(),
//       time: "2022-10-12".to_string(),
//     }
//   }
// }

// 实现方法时也可以使用 trait bound
#[allow(dead_code)]
struct Point<T> {
  x: T,
  y: T,
}

// 只有当 T 实现了 Display 和 PartiialOrd trait 时，它的实例才拥有 _cmp_display 方法
impl<T: Display + PartialOrd> Point<T> {
  fn _cmp_display(&self) {
    if self.x >= self.y {
      println!("x >= y, x is {}", self.x);
    } else {
      println!("x < y, y is {}", self.y);
    }
  }
}

// 我们也可以为实现了其他 trait 的任意类型有条件地实现某个 trait
trait DisplaySummary {
  fn display_summary(&self) -> String;
}

// 为满足 trait bound 上的所有类型实现 trait 被称为覆盖实现 (blanket implementations)
impl<T: Summary + Display> DisplaySummary for T {
  fn display_summary(&self) -> String {
    println!("{}", self);
    self.summarize()
  }
}
