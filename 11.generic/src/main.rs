// 函数上使用泛型
fn _test_fn<T>(x: T) -> T {
  x
}

// 在枚举上使用泛型
enum _Option<T> {
  Some(T),
  None,
}

// 在结构体上使用泛型
struct Point<T> {
  x: T,
  y: T,
}

// 为泛型结构体创建方法
impl<T> Point<T> {
  fn x(&self) -> &T {
    &self.x
  }
}

// 可以为特定类型的结构体创建方法
impl Point<i32> {
  // 方法可以使用不同的泛型参数
  fn area<V>(&self, v: V) -> V {
    println!("{}", self.x * self.y);
    v
  }
}

fn main() {
  let p1 = Point { x: 1, y: 1 };
  println!("x, y in p1 are {}, {}", p1.x, p1.y);
  p1.area(&p1);

  let p2 = Point { x: 3.0, y: 5.0 };
  println!("x, y in p2 are {}, {}", p2.x(), p2.y);
}

// 泛型代码性能与具体类型代码性能相同
// 编译时会进行单态化
