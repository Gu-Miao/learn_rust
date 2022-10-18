fn main() {
  // Rust 中的迭代器是惰性的，除非调用消费迭代器的方法，否则迭代器本身没有任何效果
  // 所有迭代器都实现了 Iterator trait，它只要求实现一个 next 方法
  // next 方法每次迭代返回迭代器中的一项，返回结果包裹在 Some 里，迭代结束后再调用会返回 None
  // 可以直接在迭代器上调用 next 方法
  let v1 = vec![1, 2, 3, 4, 5];
  // 声明为可变的，因为每次调用 next 方法都会“消耗”一个元素
  let mut v1_iter = v1.iter();

  assert_eq!(Some(&1), v1_iter.next());
  assert_eq!(Some(&2), v1_iter.next());
  assert_eq!(Some(&3), v1_iter.next());

  let v1_iter1 = v1.iter();
  // for in 会取得迭代器的所有权
  for v in v1_iter1 {
    println!("{}", v);
  }
  // 报错！v1_iter1 已经失效
  // println!("{:?}", v1_iter1);

  // iter 方法，在不可变引用元素上创建迭代器
  // into_iter 方法，创建的迭代器会获得元素的所有权
  // iter_mut 方法，迭代元素的可变引用

  // 消耗性适配器，最终会把迭代器耗尽
  // 比如 sum 方法，它会取得迭代器的所有权，反复调用 next 遍历所有元素，返回总和
  let v1_iter2 = v1.iter();
  println!("sum of v1 is {}", v1_iter2.sum::<i32>());

  // 迭代器适配器，可以将迭代器转换为不同种类的迭代器
  // 链式调用，可读性高
  // 比如 map 方法，类似 JavaScript 中的 Array.prototype.map
  // 迭代器是惰性的，如果不消耗它们，那就什么都不会做，闭包中的代码也不执行
  let v1_iter3 = v1.iter().map(|x| x + 1);
  let v2: Vec<_> = v1_iter3.collect();
  println!("v2 is {:?}", v2);

  // filter 方法，常用于捕获环境
  // 类似 JavaScript 中的 Array.prototype.filter
  let v3 = vec![
    String::from("rng"),
    String::from("edg"),
    String::from("jdg"),
  ];
  let v3_have_d: Vec<String> = v3.into_iter().filter(|x| x.contains("d")).collect();
  println!("{:?}", v3_have_d);

  let counter = Counter::new();
  for i in counter {
    println!("i is {}", i); // 1 2 3 4 5
  }
}

// 自定义迭代器
struct Counter {
  count: u32,
}

impl Counter {
  fn new() -> Counter {
    Counter { count: 0 }
  }
}

// 实现 Iterator trait
impl Iterator for Counter {
  type Item = u32;

  fn next(&mut self) -> Option<Self::Item> {
    // 只能迭代到 5
    if self.count < 5 {
      self.count += 1;
      Some(self.count)
    } else {
      None
    }
  }
}

// rust 的迭代器是一种零开销抽象，不会引入额外的性能开销
