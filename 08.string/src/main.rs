fn main() {
  // Rust 核心语言层面，只有一个字符串类型，就是字符串切片 str 或 &str
  // String 类型来自标准库，可增长，可修改，可拥有，是 byte 的集合
  // UTF-8 编码
  // 其他字符串类型以 String 结尾的，可获得所有权；以 Str 结尾的，可借用
  // 很多 vector 的操作也适用于 String
  let mut s1 = String::new();

  // to_string() 方法适用于实现了 Display trait 的类型，包括字符串字面值
  let s2 = "hello ".to_string();

  // 或者可以用 String::from
  let s3 = String::from("world");

  // 使用 pus_str 方法将字符串切片附加到 String 上
  s1.push_str("s1");

  // push 方法只能附加单个字符，也就是 char 类型
  s1.push('1');

  // 拼接字符串
  // + 后的变量要求是字符串切片类型
  // 使用 + 拼接字符串本质是调用了 add 方法，它的签名是这样的
  // fn add(self, s: &str) -> String
  // 可以看到，他会获得本身的所有权，且 s 的类型须为字符串切片
  // 但这里我们的 &s3 不是 &str 类型而是 &String，那么为什么不报错呢？
  // 因为这里用了一种叫做解引用强制转换 (deref coercion) 的技术，将字符串的
  // 引用转为字符串切片，所以编译通过
  let s4 = s2 + "-" + &s3;
  println!("{}", s4);
  // println!("{}", s2); // 报错，s2 的所有权已经转移给了 s4

  // 或者使用 format! 宏
  let s5 = format!("{}-{}-{}", s1, s3, s4);
  println!("{}", s5);
  println!("{}, {}", s3, s4); // format! 宏不会获得所有权，所有变量都可以正常使用

  // 字符串类型不能使用索引访问
  // String 本身是对 Vec<u8> 的包装
  // len 方法可以返回 String 所占的字节数
  println!("{}", String::from("abcd").len()); // 4，每个字母 1 字节
  println!("{}", String::from("你好").len()); // 6，每个汉字 3

  // 可以看到，当字符串中含有汉字时，每个汉字占 3 个字节。如果 Rust 中允许使用
  // 索引方式访问字符串，那么 s[0] 不会返回 "你" 而是返回 228，这通常是没有意义的
  // 并且容易造成误解。

  // 字节
  let s6 = String::from("你好");
  for byte in s6.bytes() {
    println!("byte is {}", byte);
  }

  // 每个字母或汉字称为一个 unicode 标量值
  for char in s6.chars() {
    println!("char is {}", char);
  }

  // 也可以通过字形蔟的方式，这种方式比较复杂，标准库没有提供
  // Rust 不允许使用索引访问 String 的最后一个原因是，索引操作应该消耗一个常量时间 O(1)，
  // 而 String 无法保证，因为他需要遍历所有内容，来确定有多少个字符

  // 切割字符串
  // 必须按照字符边界切割，否则会恐慌
  let s7 = "你好，世界！";
  let s7_slice = &s7[0..4]; // 运行时恐慌，因为没有按照字符边界切割
  println!("s7_slice is {}", s7_slice);
}
