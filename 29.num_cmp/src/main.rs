// 可用的运算符
// > < == != >= <=
// 只能比较相同类型的数值

// 比较不同类型的数值
// 使用 as 进行强制类型转换
fn use_as() {
  let x: i32 = 10;
  let y: i64 = 100;

  // 将比较小的类型转换为比较大的类型是安全的
  if x < (y as i32) {
    println!("x < y");
  }

  // 将比较大的类型转为比较小的类型则是不安全的，
  // 因为数字可能不被包括在比较小的类型中
  let m = 103000;
  println!("{}", m as i8);
}

// 也可以使用 try_into 进行类型转换
fn use_try_into() {
  let x: i32 = 100;
  let y: u8 = 5;

  // 此方法来自 std::convert::TryInto trait，最新版不需引入，旧版本 Rust
  // 可能需要手动引入
  let y = y.try_into().unwrap();

  if x > y {
    println!("x is {}, y is {}, x > y", x, y);
  }
}

// 浮点数的比较
// 浮点类型所代表的数字是一个近似值，因为浮点类型是基于二进制实现的，
// 但我们通常使用十进制计算
// f32 和 f64 只实现了 std::cmp::PartialEq，而其他类型还实现了 std::cmp::Eq
// 针对浮点类型比较需遵循的指导方针
// 1. 避免测试浮点类型的相等性
// 2. 如果结果在数学上属于未定义的，需要小心
fn cmp_float() {
  // JavaScript 中的经典问题，panic!
  // assert!(0.1 + 0.2 == 0.3);

  let x: (f32, f32, f32) = (0.1, 0.2, 0.3);
  let y: (f64, f64, f64) = (0.1, 0.2, 0.3);

  println!("\nx(f32)");
  println!("  0.1 + 0.2: {:x}", (x.0 + x.1).to_bits());
  println!("        0.3: {:x}\n", x.2.to_bits());

  println!("y(f64)");
  println!("  0.1 + 0.2: {:x}", (y.0 + y.1).to_bits());
  println!("        0.3: {:x}\n", y.2.to_bits());

  assert!(x.0 + x.1 == x.2);

  // panic
  // assert!(y.0 + y.1 == y.2);

  // 测试数学运算是否在真实数学结果的可接受范围内更安全，这个边界通常被称为 ε（依普西隆）
  // Rust 提供了一些可容忍的误差值 f32:EPSILON 和 f64:EPSILON
  let diff = (y.2 - (y.0 + y.1)).abs();

  if diff < f64::EPSILON {
    println!(
      "\ny.0 + y.1 is {}, y.2 is {}\ndiff is {:x} and it is less than ε({:x})\nso y.0 + y.1 is equal to y.2",
      y.0 + y.1,
      y.2,
      diff.to_bits(),
      f64::EPSILON.to_bits()
    );
  }
}

// NaN (Not A Number)
// 表示不是一个数字
// 几乎所有与 NaN 进行交互的操作都会返回 NaN
// NaN 不与任何值相等，与另一个 NaN 也不相等
fn nan() {
  let x = (-40f32).sqrt();

  // NaN
  println!("{}", x);

  // NaN 和 NaN 也不想等
  if x != x {
    println!("NaN is not equal to NaN");
  }

  // is_nan 方法可以判断是否为 NaN
  println!("{}", x.is_nan());

  let y: f64 = 1.0 / 0.0;
  println!("y is {}", y);

  println!("y: {}", y.is_finite());
}

fn main() {
  use_as();
  use_try_into();
  cmp_float();
  nan();
}
