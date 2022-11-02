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

// 特性一：解引用原始指针
fn deref_raw_pointer() {
  let mut num = 10;

  let r1 = &num as *const i32;
  let r2 = &mut num as *mut i32;

  unsafe {
    println!("*r1: {}", *r1);
    println!("*r2: {}", *r2);
  }

  // 任意的内存地址
  let address = 0x012345usize;
  let r = address as *const i32;

  unsafe {
    // 非常可能出现内存错误
    // println!("*r: {}", *r);
  }
}

use std::slice;

// 将 unsafe 代码包裹在安全函数中，对外提供安全的 API
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
  let len = slice.len();
  let ptr = slice.as_mut_ptr();

  // 特性二：调用 unsafe 函数/方法，一般需要满足一些条件（文档）
  // 需要在 unsafe 块中调用
  unsafe {
    (
      slice::from_raw_parts_mut(ptr, mid),
      slice::from_raw_parts_mut(ptr.add(mid), len - mid),
    )
  }
}

fn call_safe_fn() {
  let mut arr = [1, 2, 3, 4, 5, 6];

  // split_at_mut 是安全的，可以放心调用
  let (slice1, slice2) = split_at_mut(&mut arr, 3);

  println!("slice1: {:?}, slice2: {:?}", slice1, slice2);
}

// 使用 extern 函数调用外部代码
// 外部函数接口 (FFI, Foreign Function Interface)，它允许一种编程语言定义函数供其他编程语言调用
// 其他语言不会遵守 Rust 的规则，而 Rust 也无法对其进行检查，因而外部代码必定是不安全的
// "C" 指的是外部函数使用的应用二进制接口 (Application Binary Interface, ABI)，它用于定义函数
// 在汇编层面的调用方式。"C" ABI 是最常见的 ABI，它遵循 C 语言的 ABI
extern "C" {
  // 想要调用的外部函数的签名
  fn abs(input: i32) -> i32;
}

// 从其他语言调用 Rust 函数
// 使用 extern 关键字并指明 ABI，再添加 #[no_mangle] 注解，避免 Rust 编译器在编译时修改函数名称，
// 让其他语言正常识别函数
// 这种函数不需要使用 unsafe 关键字
#[no_mangle]
pub extern "C" fn rust_fn() {
  println!("This is from rust");
}

fn call_extern() {
  unsafe {
    println!("{}", abs(1));
  }
}

// 全局（静态）变量
// 使用 static 关键字，用大写的蛇形命名法命名，声明是必须指明其类型，生命周期必须为 'static，可以省略
// 访问不可变的静态变量是安全的
static mut NUM_COUNT: i32 = 0;

// 静态变量与常量的区别
// 1. 静态变量有固定的内存地址，使用它的值总会访问同样的数据；而常量则可以复制
// 2. 静态变量是可变的，访问或修改可变的静态变量是 unsafe 的，因为访问和修改静态变量可能出现数据竞争，
// 比如多线程场景
fn static_variable() {
  // 特性三：访问和修改可变的静态变量
  unsafe {
    NUM_COUNT += 1;
    println!("NUM_COUNT: {}", NUM_COUNT);
  }
}

// 特性四：实现 unsafe trait
// 当一个 trait 中至少存在一个方法有编译器无法校验的不安全因素时，就称这个 trait 是不安全的
// 声明和实现都需要使用 unsafe 关键字
unsafe trait Foo {
  // ...
}
unsafe impl Foo for i32 {
  //...
}

fn main() {
  deref_raw_pointer();
  call_safe_fn();
  call_extern();
  static_variable();
}
