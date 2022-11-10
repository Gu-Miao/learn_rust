// 宏 macro
// 在 Rust 中，宏指的是一组相关特性的集合称谓：
// - 使用 macro_rules! 构建的声明宏（declarative macro）
// - 3 种过程宏
//    * 自定义 #[derive] 宏，用于结构体或者枚举，可以为其指定随 derive 属性添加的代码
//    * 类似属性的宏，在任何条目上添加自定义属性
//    * 类似函数的宏，看起来像函数调用，对其指定为参数的 token 进行操作

// 宏与函数的区别
// 宏本质上是一种元编程（meta programming）
// 函数定义签名时，必须声明参数个数与类型，而宏可以处理可变的参数
// 编译器会在解释代码前展开宏
// 相比函数，宏的定义复杂的多，可读性差，难以维护
// 在某个文件调用宏之前，必须提前定义宏或者将宏引入当前作用域

// macro_rules!（可能将会弃用）
// 声明宏，类似 match 的模式匹配

#[macro_export]
macro_rules! my_vec {
  ( $( $x:expr ),* ) => {
    {
      let mut v = Vec::new();
      $(
        v.push($x);
      )*
      v
    }
  };
}

fn use_my_vec() {
  println!("A vector created by my_vec!: {:?}", my_vec![1, 2, 3, 4]);
}

// 基于属性来生成代码的过程宏
// 这种形式更像函数，接受并操作输入的 Rust 代码，返回另一些 Rust 代码
// 三种过程宏
// 1. 自定义派生宏
// 2. 属性宏
// 3. 函数宏
// 创建过程宏时，宏定义必须放在它们自己的包中，并使用特殊的包类型
// 过程宏定义必须在单独的包中，并且在 Cargo.toml 中启用 proc-macro 选项，见 (../../hello_macro)
use hello_macro::HelloMacro;

trait HelloMacro {
  fn hello_macro() {}
}

#[derive(HelloMacro)]
struct Cake;

fn hello_macro_cake() {
  Cake::hello_macro();
}

// 属性宏与自定义派生宏类似
// 允许创建新的属性，但不是为 derive 属性生成代码
// 属性宏更加灵活，derive 宏只能用于结构体和枚举，属性宏可以用于任何条目，比如函数

// #[route(GET, "/")]
// fn index() {}

// 编写方式大体与自定义派生宏相同，需要放在单独的包中，attr 对应 GET 和 "/"，item 对应 index 函数体
// #[proc_macro_attribute]
// pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {}

// 函数宏
// 类似函数调用的宏，但比函数灵活，可以接受 TokenStream 作为参数

// let sql = sql!(SELECT * FROM posts where id=1);

// #[proc_macro]
// pub fn sql(input: TokenStream) -> TokenStream {}

fn main() {
  use_my_vec();
  hello_macro_cake();
}
