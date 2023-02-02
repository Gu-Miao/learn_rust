// 函数指针
// 可以将函数传递给其他函数
// 函数在传递过程中会强制转换为 fn 类型（函数指针，function pointer）
fn add_one(i: i32) -> i32 {
  i + 1
}
fn add_twice(f: fn(i: i32) -> i32, i: i32) -> i32 {
  f(i) + f(i)
}
fn add_it() {
  let x = 5;
  println!("add_it: {}", add_twice(add_one, x));
}

// 函数指针是一个类型，而非一个 trait
// 即它可以直接用作参数类型，而不需要指定一个泛型参数将其约束为 Fn trait
// 函数指针实现了闭包的三种 trait (Fn, FnMut, FnOnce)
// 因此总是可以将函数指针作为参数传递给一个接受闭包的函数
// 所以，倾向于使用闭包 trait 的泛型来编写函数，这样既可以接受闭包，也可以接受函数
// 某些场景可能只支持接受 fn 而不接受闭包，比如和外部不支持闭包的代码交互（C 函数）
fn fn_or_closure() {
  let v = vec![1, 2, 3, 4];

  // map 方法的函数签名使用了以 FnMut 约束的泛型参数，因此既可以传递闭包，又可以传递函数
  let vs: Vec<String> = v.iter().map(|x| x.to_string()).collect();
  println!("vs: {:?}", vs);

  let vs: Vec<String> = v.iter().map(ToString::to_string).collect();
  println!("vs: {:?}", vs);
}

#[derive(Debug)]
struct Timer(u32);

#[derive(Debug)]
enum Person {
  Male(u32),
}

// 元组结构体和元组枚举变体的构造器也被实现成了一个函数，它接受对应的参数返回一个类型实例
// 因此可以将其作为函数指针使用
fn tuple_like_type_constructors() {
  let timer_list: Vec<Timer> = (0u32..10).map(Timer).collect();
  println!("timer_lsit: {:?}", timer_list);

  let person_list: Vec<Person> = (0u32..10).map(Person::Male).collect();
  println!("person_list: {:?}", person_list);
}

// 返回闭包
// 闭包使用 trait 进行表达，无法在函数中返回一个闭包，可以将一个实现了该 trait 的具体类型作为返回值
fn returns_closure1() -> impl Fn(i32) -> i32 {
  |x| x + 1
}

fn returns_closure2() -> Box<dyn Fn(i32) -> i32> {
  Box::new(|x| x + 1)
}

fn use_returned_closure() {
  let add_one = returns_closure1();
  println!("{}", add_one(5));

  let add_one = returns_closure2();
  println!("{}", add_one(5));
}

fn main() {
  add_it();
  fn_or_closure();
  tuple_like_type_constructors();
  use_returned_closure();
}
