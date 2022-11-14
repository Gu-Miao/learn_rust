fn main() {
  // 我们将枚举的每种可能称为“变体”
  // 可以将数据附加到变体中，每个变体可以有不同的数据类型和数据量
  enum IPAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
  }

  // 枚举值
  let v4 = IPAddrKind::V4(127, 0, 0, 1);
  let v6 = IPAddrKind::V6(String::from("::1"));

  // 枚举也可以定义方法
  impl IPAddrKind {
    fn call(&self) {
      println!("call!");
    }
  }
  v4.call();
  v6.call();

  // Rust 中没有 null
  // Rust 中提供了类似 null 概念的枚举 Option<T>，来自 prelude 模块
  let _num_1 = Some(1);
  let _num_none: Option<i32> = None;

  // match
  // match 允许一个值与一系列模式进行匹配，并执行模式对应的代码
  // 模式可以是字面值、变量名或通配符等等……
  enum Coin {
    Fen,
    Jiao,
    Yuan(usize),
  }
  fn coin_in_yuan(coin: Coin) -> f64 {
    match coin {
      // 当代码较为复杂时，可以使用一个块作用域
      Coin::Fen => {
        println!("Fen!");
        0.01
      }
      Coin::Jiao => 0.1,
      // match 可以拿到变体中的数据
      Coin::Yuan(count) => {
        println!("count is {}", count);
        1.0
      }
    }
  }
  let _fen = coin_in_yuan(Coin::Fen); // 0.01
  let _jiao = coin_in_yuan(Coin::Jiao); // 0.1
  let _yuan = coin_in_yuan(Coin::Yuan(1)); // 1.0

  // match 匹配必须穷尽所有的可能，以确保代码的合法有效
  // 如果可能的值太多，可以使用 _ 通配符
  let v = 0u8;
  match v {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    _ => (),
  }

  // if let 只关心一种匹配并忽略其他匹配的情况
  if let 3 = v {
    println!("three");
  } else {
    println!("others");
  }
}
