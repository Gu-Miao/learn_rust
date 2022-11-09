// 泛型 trait
trait Pay<T> {
  fn pay(&self) -> T;
}

struct Dollar {
  value: u32,
}

// 实现泛型 trait 必须指明泛型参数的类型
impl Pay<u32> for Dollar {
  fn pay(&self) -> u32 {
    self.value
  }
}
// 可以通过传入不同的泛型参数来多次实现
impl Pay<Option<u32>> for Dollar {
  fn pay(&self) -> Option<u32> {
    Some(self.value)
  }
}

// 在 trait 定义中使用关联类型来指定占位符
// 关联类型 (associated type) 是 trait 中的类型占位符，它可以用于 trait 的方法签名中
// 可以定义出包含某些类型的 trait，在实现 trait 之前不需要知道这些类型是什么
trait Sell {
  type Price;

  fn sell(&self) -> Self::Price;
}

struct Pizza {
  price: f32,
}

// 只能实现一次
impl Sell for Pizza {
  // 标注关联类型
  type Price = f32;

  fn sell(&self) -> Self::Price {
    self.price
  }
}

// 泛型参数默认值和运算符重载
// 可以在使用泛型参数时为泛型参数指定一个默认值，这种技术常用于运算符重载 (operator overloading)
// Rust 不允许创建自定义运算符以及重载任意的运算符，但可以通过实现 std::ops 的一些 trait 来重载部分运算符
use std::ops::Add;

#[derive(Debug)]
struct Point {
  x: i32,
  y: i32,
}

// 实现 Add trait 重载 + 运算符
// Add trait 有一个泛型参数 Rhs 默认类型为 Self
impl Add for Point {
  type Output = Point;

  fn add(self, rhs: Self) -> Self::Output {
    Point {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
    }
  }
}

fn overload_plus_for_point() {
  let p1 = Point { x: 1, y: 1 };
  let p2 = Point { x: 10, y: 10 };

  println!("p1 + p2: {:?}", p1 + p2);
}

// 修改 Add 的默认泛型参数类型实现毫米与米的相加
#[derive(Debug)]
struct Millimeters(u32);
#[derive(Debug)]
struct Meters(u32);

impl Add<Meters> for Millimeters {
  type Output = Millimeters;

  fn add(self, rhs: Meters) -> Self::Output {
    Millimeters(self.0 + (rhs.0 * 1000))
  }
}

fn overload_plus_for_meters() {
  let mm = Millimeters(1000);
  let m = Meters(1);

  println!("{:?}", mm + m);
}

// 调用同名方法
trait Pilot {
  fn fly(&self);
}
trait Wizard {
  fn fly(&self);
}

struct Human;

impl Human {
  fn fly(&self) {
    println!("Human is flying");
  }
}
impl Pilot for Human {
  fn fly(&self) {
    println!("Pilot is flying");
  }
}
impl Wizard for Human {
  fn fly(&self) {
    println!("Wizard is flying");
  }
}

fn function_with_same_name() {
  let p = Human;
  p.fly();
  Pilot::fly(&p);
  Wizard::fly(&p);
}

// 完全限定语法
trait Animal {
  fn baby_name() -> String;
}

struct Dog;

impl Dog {
  fn baby_name() -> String {
    String::from("Qiu Qiu")
  }
}

impl Animal for Dog {
  fn baby_name() -> String {
    String::from("Dixi")
  }
}

fn fully_qualified_syntax() {
  println!("{}", Dog::baby_name());
  println!("{}", <Dog as Animal>::baby_name());
}

// 使用 supertrait 来要求 trait 附带其他 trait 的功能
// 需要在一个 trait 中使用其他 trait 的功能：
// - 需要被依赖的 trait 被实现
// - 那个间接被依赖的 trait 就是当前 trait 的 supertrait
use std::fmt::Display;
trait DisplayX: Display {
  fn print(&self) {
    let content = self.to_string();
    println!("x: {}", content);
  }
}

struct Road {
  name: String,
  directrion: String,
}

impl Display for Road {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "This is {}, direction is {}.",
      self.name, self.directrion
    )
  }
}

impl DisplayX for Road {}

fn print_a_road() {
  let road = Road {
    name: String::from("LeKai South Street"),
    directrion: String::from("south"),
  };
  road.print();
}

// 使用 newtype 绕过孤儿规则为外部类型实现外部 trait
// 孤儿规则：只有 trait 或类型定义在本地包时，才能为这个类型实现该 trait
// 通过元组结构体将类型“包一层”来实现
struct Wrapper(Vec<String>);

impl Display for Wrapper {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "[{}]", self.0.join(", "))
  }
}

fn print_wrapped_vec() {
  let v = Wrapper(vec![
    String::from("a"),
    String::from("b"),
    String::from("c"),
  ]);
  println!("{}", v);
}

fn main() {
  overload_plus_for_point();
  overload_plus_for_meters();
  function_with_same_name();
  fully_qualified_syntax();
  print_a_road();
  print_wrapped_vec();
}
