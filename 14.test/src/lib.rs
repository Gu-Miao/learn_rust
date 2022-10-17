// 测试
// 一个测试就是一个函数，验证代码是否如预期执行
// 测试函数需要使用 #[test] 属性标注

// 测试函数体通常会执行的三个操作
// 1. 准备数据/状态
// 2. 运行被测试代码
// 3. 断言

// 使用 cargo test 运行所有测试函数

pub fn add(left: usize, right: usize) -> usize {
  left + right
}

// 可以有很多 test module
// 可以使用 pravite 接口
// 一般放到 src 目录下的同一个文件中，约定俗称的，每个文件需要创建一个名为 tests 的模块
// 表明这是单元测试模块
// 单元测试一般使用 #[cfg(test)] 标注
// 表示被标注的条目只有在 test 时才被编译，test 由 Rust 提供，用来编译运行测试
#[cfg(test)]
mod tests {
  use super::*;

  // 函数名就是测试名
  #[test]
  fn it_works() {
    let result = add(2, 2);

    // 测试相等，前两个参数位置可以任意调换
    assert_eq!(result, 4,);

    // 测试不相等，同上
    assert_ne!(result, 5);

    // assert_eq 和 assert_ne 这两个宏使用 debug 格式打印参数
    // 要求参数实现了 PartialEq 和 Debug 这两个 trait (标准库
    // 中所有基本类型和大部分类型都实现了)
  }

  #[test]
  // should_panic 宏用来检查程序是否如预期一样发生了恐慌
  // 可以添加一个信息让 should_panic 更精确，这会使它检查 panic 中
  // 的消息是否包含指定的文字
  // 下面的代码也可以这么写：#[should_panic(expected = "panic")]
  #[should_panic = "panic"]
  fn it_panic() {
    // assert 宏接受一个布尔类型，如果成功则测试成功
    // 否则会调用 panic 宏使测试失败
    assert!(5 > 3);

    // 可以使用 panic 宏使测试恐慌 (失败)
    panic!("panic");
  }

  #[test]
  fn it_custom_message() {
    // assert, assert_eq 和 assert_ne 这三个宏可以添加自定义错误信息，
    // 自定义错误信息为可选参数，它会被传递给 format 宏，因此可以使用占位
    // 符 {}，并且后面跟上其他参数

    let v1 = 4;
    let v2 = 5;
    let v3 = v2 - 1;
    assert_eq!(v1, v3, "{} is not equal to {}", v1, v3);
    assert_ne!(v1, v2, "{} is equal to {}", v1, v2);
    assert!(v2 > v1, "{} is not grater than {}", v2, v1);
  }

  #[test]
  fn it_result() -> Result<(), String> {
    if 2 + 2 == 4 {
      Ok(())
    } else {
      Err(String::from("2 + 2 is not equal to 4"))
    }
  }

  // cargo test 的默认行为
  // 1. 测试并行运行
  // 2. 运行所有测试
  // 3. 捕获 (但不显示) 所有输出，使读取与测试结果相关的输出更加容易

  // cargo test --help
  // cargo test -- --help
  // cargo test -- --test-threads=1 只有一个线程，相当于串行执行
  // cargo test -- --show-output 显式打印函数输出
  // cargo test test_name 只运行名称包含 test_name 的所有测试函数，可以执行 cargo test it 试一下

  #[test]
  // 使用 ignore attribute 可以让 cargo test 命令忽略它修饰的函数
  // 可以使用 cargo test -- --ignored 来专门运行被忽略的测试
  #[ignore]
  fn it_expensive() {
    assert_eq!(5, 1 + 1 + 1 + 1 + 1 + 1 + 1);
  }
}

// 集成测试请跳转至 tests/intergration_test.rs
