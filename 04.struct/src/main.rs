fn main() {
  #[derive(Debug)] // 派生于 Debug trait
  struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
  }

  let username = String::from("liming");

  // 声明时字段可以是乱序的，但必须指定所有字段的值
  let mut user1 = User {
    email: String::from("xxxx@xxx.com"),
    username, // 与 JavaScript 一样，可以使用简写
    sign_in_count: 0,
    active: false,
  };

  // 使用 #[derive(Debug)] 配合 {:?} 或 {:#?} 占位符打印结构体
  println!("{:?}", user1);
  println!("{:#?}", user1);

  // 点标记法访问结构体上的属性
  println!("username: {}", user1.username);
  println!("email: {}", user1.email);
  println!("sign_in_count: {}", user1.sign_in_count);
  println!("active: {}", user1.active);

  // 如果将实例指定为可变的，那么所有的字段都是可变的
  // Rust 不允许结构体一部分属性是可变的而其他是不可变的
  user1.username = String::from("Li Ming");

  // 结构体更新语法
  // 这里与 JavaScript 的 ... 不是一回事，..user1 不会覆盖 usernmae 或者
  // sign_in_count 属性，只会补充 User 剩下的属性，并且需要 user1 与 _user2
  // 是相同类型
  let _user2 = User {
    username: String::from("Xiao Hui"),
    sign_in_count: 1,
    ..user1
  };

  // Tuple Struct
  // 整体有名，属性没有名
  // 适用于想给整个元组起名，让它不同于其他元组，又不需要给每个属性起名的情况
  struct RGBA(u8, u8, u8, u8);
  let _red = RGBA(255, 0, 0, 1);

  // Unit-Like Struct
  // 可以声明没有任何属性的结构体
  // 适用于需要在某个类型上实现某个 trait，但是有没有需要存储的数据
  struct _UnitLikeOne {}

  // 结构体中的所有权
  // 如果结构体实例拥有所有数据的所有权，只要该实例有效，那么它的属性也是有效的
  // 如果机构体中存在引用，需要使用生命周期（参见后续章节）

  // 方法与关联函数
  // 通过 impl 块可以声明结构体的方法
  struct Rectangle {
    width: u32,
    height: u32,
  }
  impl Rectangle {
    // 方法的第一个参数总是 self，可以加 & 或 mut
    fn area(&self) -> u32 {
      self.width * self.height
    }
  }
  let rect = Rectangle {
    width: 100,
    height: 100,
  };
  // 调用方法时，Rust 会根据情况自动添加 &, &mut 或 *，以便于匹配方法的签名
  print!("area of rect is {}", rect.area());

  // impl 块可以有多个
  impl Rectangle {
    // 如果第一个参数不是 self，那么这就是一个关联函数
    // 方法与关联函数可以理解为 JavaScript 中的原型方法和静态方法
    fn create(width: u32, height: u32) -> Rectangle {
      Rectangle { width, height }
    }
  }

  let _rect = Rectangle::create(10, 10);
}
