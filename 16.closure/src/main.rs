use std::{collections::HashMap, thread, time::Duration};

// 闭包是可以捕获所在环境的匿名函数
// 1. 匿名函数
// 2. 保存变量或作为参数
// 3. 可以在一个地方创建闭包，然后在另一个地方调用闭包进行计算
// 4. 可以从其定义的作用域内捕获值

// 闭包不要求显式地标注参数和返回值类型
// 闭包通常比较短小，只在狭窄的上下文中工作，编译器往往可以可靠地推断出它的类型

fn main() {
  // 闭包可以访问作用域中的变量，而函数则不能
  let x = 1;
  let y = 4;

  // 使用闭包读取了 x
  // 捕获环境会产生额外的内存开销，函数则没有
  let equal_to = |v| v == x;
  assert!(equal_to(y));

  // 使用函数会报错
  // fn eual_to_fn(v: i32) -> bool {
  //   v == x
  // }
  // assert!(eual_to_fn(y));
  // 从这里也能看出，JavaScript 中的所有函数都是闭包

  generate_workout(10, 7);

  // move 关键字强制闭包取得它所使用环境的所有权
  // 有用的场景：将闭包传递给新线程并移动数据令其归新线程所有
  let str = String::from("hello!");
  let is_greater = move |value| value > str.len();
  // 报错！所有权已经移动给了 is_greater 闭包
  println!("str is {}", str);
  assert!(is_greater(5));
}

// 每个闭包都有自己唯一的匿名类型，即使两个闭包签名完全一致
// 所有闭包至少实现了一个 Fn trait (标准库提供)
// Fn     不可变借用
// FnMut  可变借用
// FnOnce 获取所有权
// 创建闭包时，通过闭包对环境值的使用，Rust 可以推断出具体使用了哪个 trait
// 所有的闭包都实现了 FnOnce
// 没有移动捕获变量的实现了 FnMut
// 无需可变访问捕获变量的实现了 Fn
// 当指定 trait bound 时，优先使用 Fn，如果需要使用另外两种，编译器会有提示
struct Cacher<T>
where
  T: Fn(u32) -> u32,
{
  calculator: T,
  values: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
  T: Fn(u32) -> u32,
{
  fn new(calculator: T) -> Cacher<T> {
    Cacher {
      calculator,
      values: HashMap::new(),
    }
  }
  fn value(&mut self, intensity: u32) -> u32 {
    match self.values.get(&intensity) {
      Some(value) => *value,
      None => {
        let value = (self.calculator)(intensity);
        self.values.insert(intensity, value);
        value
      }
    }
  }
}

fn generate_workout(intensity: u32, random_number: u32) {
  let mut cacher = Cacher::new(|num| {
    println!("calculating...");
    thread::sleep(Duration::from_secs(2));
    num
  });

  if intensity < 25 {
    println!("First, let's do {} pushups!", cacher.value(intensity));
    println!("Next, do {} situps!", cacher.value(intensity + 5));
  } else {
    if random_number == 3 {
      println!("Have a rest today!");
    } else {
      println!("run fro {} minutes!", cacher.value(intensity));
    }
  }
}
