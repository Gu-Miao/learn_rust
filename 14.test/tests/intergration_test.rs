use my_test::add;

mod common;

// tests 目录会被 cargo 特殊对待，不需要使用 cfg attribute 标注
// tests 目录下的每个文件会被视为一个 crate
// cargo test --test 文件名 可以运行特定文件的全部测试 cargo test --test intergration_test

#[test]
fn it_add() {
  common::helper();
  assert_eq!(4, add(3, 1));
}

// 如果项目是一个 binary crate，只含有 src/main.rs 没有 src/lib.rs
// 那么就不能在 tests 下创建集成测试，无法把 main.rs 导入
// 一般通常会将逻辑放入 lib.rs，然后 main.rs 只放入少量胶水代码
