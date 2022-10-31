// 模式
// 模式是 Rust 中一种特殊的语法，于用匹配类型的结构
// 模式由以下元素（的一些组合）组成：
// * 字面值
// * 解构的数组，结构体，枚举和元组
// * 变量
// * 通配符
// * 占位符
// 想要使用模式，需要将其与某个值进行比较，如果匹配成功，就可以使用这个值的对应部分

// match 的分支
// match 要求详尽的匹配（即匹配所有可能性）
// 特殊的 _ 模式，它匹配任何值，但不会绑定数据，通常用于最后一个分支用于忽略某些可能的情况

// if let 表达式
// if let 是用来代替只有一个分支的 match 的简写
// 可选的拥有 else, else if 和 else if let，且它不会检查穷尽性

fn complex_if_let() {
  let color: Option<&str> = None;
  let is_sunday = false;
  let age: Result<u8, _> = "22".parse();

  // 其实可以把 if let 看作一个普通的 if，let Some(color) = color 看作 if 的条件
  if let Some(color) = color {
    println!("color: {}", color)
  } else if is_sunday {
    println!("today is sunday!");
  } else if let Ok(age) = age {
    println!("age is {}", age);
  } else {
    println!("The final arm");
  }
}

fn while_let() {
  let mut v = vec![1, 2, 3];

  while let Some(top) = v.pop() {
    println!("{}", top);
  }
}

fn other_patterns() {
  let v = vec![0, 10, 20];

  // for 后的是模式
  for (i, value) in v.iter().enumerate() {
    println!("{}: {}", i, value);
  }

  // let 语句
  // let PATTERN = EXPRESSION;
  let (_a, _b, _c) = (1, 2, 3);
}

// 模式有两种：可辨驳的，不可辩驳的
// 能匹配任何可能传递的值的模式，就是不可辩驳的，比如：let x = 5;
// 有些可能失败的，就是可辨驳的：if let Some(x) = x_value
// 函数参数，if 语句，for 循环只接受不可辩驳的模式
// if let 或 while let 接受两种模式

// 模式语法
fn pattern_grammer() {
  // 匹配字面值
  let x = 1;

  match x {
    1 => println!("x is {}", 1),
    2 => println!("x is {}", 2),
    _ => println!("x is not 1 or 2"),
  }

  // 匹配命名变量
  let y = Some(5);

  match y {
    Some(y) => println!("matched: {}", y),
    _ => {}
  }

  // 多重模式，使用 | 符号
  match x {
    1 | 2 => println!("x is {}", x),
    _ => println!("x is not 1 or 2"),
  }

  // 使用 ..= 匹配一个范围
  let z = 3;
  match z {
    1..=3 => println!("x is {}", z),
    _ => println!("x is not 1 or 2"),
  }

  let g = 'g';
  match g {
    'a'..='f' => println!("in a~f"),
    'g'..='o' => println!("in g~o"),
    _ => println!("Not in a~o"),
  }

  // 解构
  #[derive(Debug)]
  struct Point {
    x: i32,
    y: i32,
  }

  // 解构结构体，可以通过 : 设置别名
  let Point { x, y: yy } = Point { x: 2, y: 5 };
  println!("Postion of point x: {}, y: {}", x, yy);

  let point = Point { x: 0, y: 10 };

  // 匹配结构体中的值
  match point {
    Point { x: 0, .. } => println!("x of point is 0, {:?}", point),
    _ => {}
  }
}

fn main() {
  complex_if_let();
  while_let();
  other_patterns();
  pattern_grammer();
}
