// unsafe rust 存在的原因：
// 1. 静态分析是非常保守的
// 2. 计算机硬件本身就是不安全的，Rust 需要进行底层系统编程

// 注意：
// 1. unsafe 没有停用如借用检查等安全检查，只是可以使用四种编译器不会进行安全检查的特性，
// 因此使用 unsafe Rust 实际上可以获得一定程度的安全保障
// 2. 内存相关的错误必须留在 unsafe 代码块中
// 3. 可以将 unsafe 代码块放在安全的抽象中以提供安全的 API

// 原始指针，又称裸指针 (raw pointer)
// 可变的 *mut T
// 不可变的 *const T，解引用后不能对其进行赋值
// 与引用的不同点：
// 1. 原始指针可以忽略借用规则，即允许同时拥有指向同一位置的多个可变和不可变指针
// 2. 不能保证指向合理内存
// 3. 允许为 null
// 4. 不实现自动清理
// 为什么使用原始指针？
// 1. 与 C 语言交互
// 2. 构建借用检查器无法理解的安全抽象

// 解引用原始指针
fn deref_raw_pointer() {
  let mut num = 10;

  let r1 = &num as *const i32;
  let r2 = &mut num as *mut i32;

  unsafe {
    println!("*r1: {}", *r1);
    println!("*r2: {}", *r2);
  }

  let address = 0x012345usize;
  let r = address as *const i32;

  unsafe {
    println!("*r: {}", *r);
  }
}

// 调用 unsafe 函数/方法，一般需要满足一些条件（文档）
// 需要在 unsafe 块中调用
unsafe fn danger() {}
fn call_unsafe() {
  unsafe {
    danger();
  }
}

fn main() {
  deref_raw_pointer();
  call_unsafe();
}
