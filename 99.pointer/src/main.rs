// 什么是指针？
//
// 指针是一种计算机引用无法立即直接访问的数据的一种方式。
//
// * 数据在物理内存（RAM）中是分散存储的。
// * 地址空间是检索系统。
// * 指针就被编码为内存地址，使用 usize 类型表示。一个地址就会指向地址空间中的某个地方。
// * 地址空间的范围是 OS 和 CPU 提供的外观界面。程序只知道有序的字节序列，不会考虑系统中实际的 RAM 数量。

// 名词解释
//
// * 内存地址，就是指代内存中单个字节的一个数。内存地址是由汇编语言提供的抽象。
// * 指针（有时被拓展称为原始指针），就是指向某一类型的一个内存地址。指针是由高级语言提供的抽象。
// * 引用，就是指针。如果是动态大小的类型，就是指针和具有额外保证的一个整数。引用是 Rust 提供的抽象。

// Rust 的引用
//
// * 引用始终引用的有效数据。
// * 引用与 usize 的倍数对齐。当 CPU 在读取未对齐的内存时，操作的速度较慢。为了缓解此问题，Rust 的类型系统会填充
//   一些字节来保证内存是对齐的，从而保证系统的运行速度。
// * 引用可以为动态大小的类型提供上述的保障。对于在内存中没有固定大小的类型，Rust 会保证其长度（或称为宽度），会保
//   存在内部指针的附近。如此，Rust 可以保证引用永远不会超出类型在内存中的空间。

fn ram_test_1() {
  static B: [u8; 10] = [99, 55, 66, 54, 15, 94, 53, 64, 78, 15];
  static C: [u8; 11] = [156, 65, 66, 75, 33, 76, 61, 62, 178, 105, 0];

  let a: i32 = 42;
  let b = &B;
  let c = &C;

  println!("===== ram_test 1 =====");

  // {:p} 这种格式化代表我们需要打印的东西是一个指针，并且我们需要打印其地址。
  println!("a: {}, b: {:p}, c: {:p}\n", a, b, c);

  // 内存参考图与详解建议观看视频第一分 P
  // https://www.bilibili.com/video/BV1TF411P74m?p=1 5:00 处

  // 变量 a 是 i32 类型，占用 4 字节。
  // 变量 b 和 c 是引用，在 32 位上是 4 字节，在 64 位上是 8 字节（因为引用是 usize 类型）。
  // b 模拟的是智能指针，c 模拟的是原始指针

  // 按视频中每个内存单元为 16 位二进制表示，三个变量内存占用的分析如下：

  // a
  //
  // 整数类型，实际表示为 i16，内存中为 [0, 42]

  // b
  //
  // 智能指针类型，[0,     10,     0,     32]
  //             长度字段（i16）   地址字段（u16）
  // 长度为 10 的固定宽度的 buffer，包含不带终止符的字节。
  // 当在指针类型后面使用时，buffer 通常称为后备数组。
  // b 和 B 一起几乎可以创建出 Rust 的字符串类型，但字符串类型还包括一个容量字段。

  // c
  //
  // 裸指针，表现为 [0, 16]，u16 类型，只保存地址。
  // 以 0 结尾的 buffer，在 C 语言中就是字符串的内部表示。
  // c 和 C 一起，就是 Rust 中的 CStr 类型
}

use std::mem::size_of;

fn ram_test_2() {
  static B: [u8; 10] = [99, 55, 66, 54, 15, 94, 53, 64, 78, 15];
  static C: [u8; 11] = [156, 65, 66, 75, 33, 76, 61, 62, 178, 105, 0];

  let a: usize = 42;
  let b: Box<[u8]> = Box::new(B);
  let c = &C;

  println!("===== ram_test 2 =====");

  println!("a（无符号整数）");
  println!("地址：{:p}", &a);
  println!("大小：{:?} bytes", size_of::<usize>());
  println!("值：{:?}\n", a);

  println!("b（放入 Box 中）");
  println!("地址：{:p}", &b);
  println!("大小：{:?} bytes", size_of::<Box<[u8]>>());
  println!("值：{:p}\n", b);

  println!("c（C 的引用）");
  println!("地址：{:p}", &c);
  println!("大小：{:?} bytes", size_of::<&[u8; 11]>());
  println!("值：{:p}\n", c);

  println!("B（10 bytes 的数组）");
  println!("地址：{:p}", &B);
  println!("大小：{:?} bytes", size_of::<[u8; 10]>());
  println!("值：{:?}\n", B);

  println!("C（11 bytes 的数组）");
  println!("地址：{:p}", &C);
  println!("大小：{:?} bytes", size_of::<[u8; 11]>());
  println!("值：{:?}\n", C);
}

use std::borrow::Cow;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::println;

fn ram_test_3() {
  static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
  static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];

  let a = 42;
  let b: String;
  let c: Cow<str>;

  let b_ptr = &B as *const u8 as *mut u8;
  let c_ptr = &C as *const u8 as *const c_char;

  unsafe {
    b = String::from_raw_parts(b_ptr, 10, 10);
    c = CStr::from_ptr(c_ptr).to_string_lossy();
  }

  println!("===== ram_test 3 =====\n");

  println!("a: {}, b: {}, c: {}", a, b, c);
}

// 原始指针
//
// 原始指针（裸指针）是指没有 Rust 标准保障的内存地址，它们是 unsafe 的。
// 类型声明：
// 可变：*const T
// 不可变：*mut T
// 示例：*const String 就是表示指向字符串的不可变原始指针。
//
// * *const T 与 *mut T 之间差异很小，可以相互转换，上面 ram_test_3 函数中就进行了转换。
// * Rust 中的引用 &T 和 &mut T 最终都会被编译为原始指针，这就意味着我们无需冒着 unsafe
//   的风险，就可以获得原始指针的性能。

fn main() {
  ram_test_1();
  ram_test_2();
  ram_test_3();
}
