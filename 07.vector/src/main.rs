fn main() {
  // Vec::new 创建空 vector 我们可以显式地指明其类型
  // vector 中所有元素类型需要相同
  let _v1: Vec<i32> = Vec::new();

  // 可以使用 vec! 宏
  let _v2 = vec![1, 2, 3];

  // 当我们调用 push 方法向 vector 中添加元素时，Rust 编译器就可
  // 以自动推断它的类型
  let mut v3 = Vec::new();
  v3.push(1);

  // 读取 vector 中的元素
  // 通过索引访问，但如果索引访问越界，那么程序会恐慌
  // 可以试试将下面代码改为 1，再执行，看看会输出什么
  println!("{}", v3[0]);

  // 使用 get 方法配合模式匹配
  match v3.get(1) {
    Some(item) => println!("item is {}", item),
    None => println!("Nothing found"),
  }

  // 同一作用域内不能同时存在可变引用和不可变引用，此规则对 vector 也适用
  let mut v4 = vec![1, 2, 3, 4];

  // 不可变引用
  let first = &v4[0];

  // push 方法的中的 self 是可以变引用，
  // 这时同一作用域内同时存在可变引用与不可变引用，报错
  // d.push(5);

  println!("first is {}", first);

  // vector 的遍历
  for i in &mut v4 {
    *i += 100; // 解引用并修改 vector 中的值
    println!("{}", i);
  }

  // 可以使用附加数据的枚举类型来让 vector 携带不同类型的数据
  enum Sheet {
    Int(i32),
    Float(f64),
    Text(String),
  }
  let _v5 = vec![
    Sheet::Int(1),
    Sheet::Float(3.14),
    Sheet::Text(String::from("some text")),
  ];
}
