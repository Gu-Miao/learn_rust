fn main() {
  // 所有权
  // 每个值都有一个变量，变量是该值的所有者
  // 每个值同时只能有一个所有者
  // 所有者离开作用域时，Rust 自动调用 drop 函数删除该值
  fn _some_fn() {
    // _x 未声明，不可用
    let _x = 1; // _x 可用
                // 可用对 _x 进行操作
  } // _x 离开作用域，失效

  // String
  // 字符串字面值在编译时就知道其内容，文本内容会被硬编码到最终的可执行文件中。速度快，不可变
  // String 存储在堆上，可以存储编译时大小未知的文本
  // 可以使用 from() 函数通过字符串字面值生成 String 类型
  let _string_from = String::from("Hello World");

  // 移动 (Move)
  // 将堆上的变量作为值赋给新变量时会发生移动 (Move)，所有权发生转移
  let str1 = String::new();
  let _str2 = str1; // 发生移动，str1 失效

  // 下一行会报错
  println!("str1 is {}", str1);

  // 如果一个类型实现了 Copy trait，那么旧变量在赋值给新变量后仍然可用，这是因为发生了复制
  // 标量类型都实现了 Copy trait，对于元组或数组来说，如果它们中的所有元素都是可以复制的，
  // 那么它也是可以复制的
  let num1 = 1;
  let num2 = num1;
  println!("numbers are {}, {}", num1, num2);

  let arr1 = [1, 2, 3];
  let _arr2 = arr1;
  for element in arr1.iter() {
    println!("element is {}", element);
  }

  let arr3 = [String::from("1"), String::from("2"), String::from("3")];
  let _arr4 = arr3; // 所有权转移

  // arr3 中的元素不是可以复制的，因此下面代码会报错
  for element in arr3.iter() {
    println!("element is {}", element);
  }

  // 所有权与函数
  // 在语义上，将值传给函数和赋给变量是类似的，将值传给函数要么发生移动，要么发生复制
  let num_arg = 1;
  let str_arg = String::from("1");

  fn num_str(num: i32, str: String) {
    println!("{}, {}", num, str);
  }

  num_str(num_arg, str_arg); // num_arg 被复制，str_arg 的所有权被转移

  // 报错，str_arg 已经失效
  println!("{}, {}", num_arg, str_arg);

  // 引用与借用
  // & 符号代表引用，允许使用变量的值但不获得其所有权
  // 把使用引用作为函数参数的行为称为借用 (brower)
  // 引用默认为不可变的，如果想修改，可以使用 &mut
  let mut ref_str = String::from("Hello");
  fn push_world(str: &mut String) {
    str.push_str(" World");
  }
  push_world(&mut ref_str); // 借用
  println!("ref_str is {}", ref_str); // ref_str 仍然可用

  // 可变引用有限制：特定作用域内，对于某一块数据，只能同时有一个可变引用，这是为了防止数据竞争
  // 数据竞争发生的条件：
  // 1. 两个或多个指针指向同一数据
  // 2. 至少有一个指针用于写入数据
  // 3. 没有使用任何同步机制来同步对数据的访问

  // 可以通过创建一个块作用域来避免上述问题
  let mut sss = 1;
  let _sss1 = &mut sss;
  {
    // 创建一个块作用域，_sss2 只在块作用域内生效
    let _sss2 = &mut sss;
  }

  // 另一个限制是，不可以同时拥有一个可变引用和一个不可变引用
  // 可以有多个不可变引用
  let mut xxx = String::from("xxx");
  let _xxx1 = &xxx;
  let _xxx2 = &xxx;
  let _xxx3 = &mut xxx; // 报错
  println!("{}, {}, {}", _xxx1, _xxx2, _xxx3);

  // 悬垂指针
  // 一个指针引用了内存中的某个地址，而这块内存可能已经释放并分配给其他人使用了
  // Rust 的编译器可以保证引用永远都不是悬垂引用

  // 如果下面的函数能编译通过，s 会在 dangle() 函数结束后失效，但返回的 &s 引用还有效，
  // 就会发生悬垂引用。但 Rust 的编译器不会让它通过
  fn dangle() -> &String {
    let s = String::from("some text");
    &s
  }

  // 切片 (slice)
  // 左闭右开区间，左右索引可以省略
  let some_str = String::from("some text");
  let _some = &some_str[..4]; // some
  let _text = &some_str[5..]; // text
  let _whole = &some_str[..]; // some text

  // 字符串字面值是一个切片
  // 使用字符串切片作为参数可以使 API 更加通用，且不损失任何功能
}
