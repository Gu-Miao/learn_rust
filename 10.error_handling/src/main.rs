use std::error::Error;
use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn _some_fn() {
  // 错误
  // 分为可恢复和不可恢复，比如文件未找到（可恢复），索引访问越界（不可恢复）
  // 可恢复，Rust 提供了 Result<T, E> 枚举
  // 不可恢复，可以使用 panic! 宏

  // panic! 宏执行时
  // 程序打印报错信息
  // 展开 (unwind) 并清理调用栈
  // 退出程序

  // 如果想让二进制文件更小，可以把展开改为中止
  // 查看 Cargo.toml

  // 取消下面的注释，运行试试
  // panic!("panic!!!");

  // 如果想看详细的 backtrace，可以将环境变量 RUST_BACKTRACE 设为 1
  // 想更全面的保存信息可以将其设为 full

  // 在 --release 模式下不会显式调试信息

  // Result<T, E> 枚举
  // 来自 prelude 模块
  let _file = match File::open("text.txt") {
    Ok(file) => file,
    Err(err) => match err.kind() {
      ErrorKind::NotFound => match File::create("text.txt") {
        Ok(file) => file,
        Err(err) => panic!("Can not create file, error: {:?}", err),
      },
      other_error => panic!("Can not open file, error: {:?}", other_error),
    },
  };

  // unwrap
  // 如果 Result 的结果时 Ok，就返回 Ok 中的内容
  // 如果 Result 的结果是 Err，就调用 panic! 但不能自定义错误信息
  let _file1 = File::open("text1.txt").unwrap();

  // expect 同 unwrap，但是可以自定义错误信息，我们曾在猜字游戏中用到过
}

// 传播错误
fn _read_username_from_file(path: &str) -> Result<String, io::Error> {
  let file = File::open(path);

  let mut file = match file {
    Ok(file) => file,
    Err(err) => return Err(err),
  };

  let mut s = String::new();
  match file.read_to_string(&mut s) {
    Ok(_) => Ok(s),
    Err(err) => Err(err),
  }
}

// 使用 ? 优化
// ? 符号表示如果发生了错误，即 Result 是 Err，就将错误 return，否则 Ok 中的值就是表达式中的值
// ? 符号只能用于 Result 或 Option 或是实现了 Try trait 的类型上
// 本质上是隐式地调用了 std::convert::From 上的 from 函数，它会将错误转化为函数期望的错误类型
// 如果一个错误类型 EA 实现了一个 from 函数且返回类型为错误类型 EB，那么就可以从 EA 转为 EB
fn _read_username_from_file1(path: &str) -> Result<String, io::Error> {
  let mut file = File::open(path)?;

  let mut s = String::new();
  file.read_to_string(&mut s)?;

  Ok(s)
}

// 通过链式调用进一步优化
fn _read_username_from_file2(path: &str) -> Result<String, io::Error> {
  let mut s = String::new();

  File::open(path)?.read_to_string(&mut s)?;

  Ok(s)
}

// main 函数中也可以使用 ? 符号
// main 函数的返回类型可以是 Result
// Box<dyn Error> 是一个 trait 对象，可以简单理解为“任何可能的错误”
fn main() -> Result<(), Box<dyn Error>> {
  let _file = File::open("xxx.text")?;
  Ok(())
}

// 何时使用 panic!
// 在定义一个可能失败的函数时，优先考虑使用 Result，如果某种情形不可恢复，
// 可以考虑使用 panic!

// 适合 panic 的场景
// 1. 演示某些概念 unwrap
// 2. 原型代码 unwrap expect
// 3. 测试

// 有时你比编译器掌握更多信息，类似 TypeScript 中的类型断言，你知道这个代码一定是正确/错误的

// 调用代码，传入无意义/错误参数
// 调用外部不可控代码
// 对值进行校验
