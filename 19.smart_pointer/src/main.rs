use std::ops::Deref;
use std::rc::Rc;

fn main() {
  // 智能指针
  // 引用只是借用数据，而智能指针很多时候拥有它所指向的数据
  // String 和 Vec<T> 是智能指针

  // Box<T>
  // Box<T> 是只有一个元素的 tuple struct
  // 最简单的智能指针
  // 允许在堆内存上存储数据
  // 栈内存上有指向堆内存的指针
  // 没有额外性能开销，但也没有额外功能
  // 实现了 Deref trait 和 Drop trait
  //
  // 常用场景：
  // 1. 编译时，某类型大小无法确定。但使用该类型时，上下文需要知道它的具体大小
  // 2. 有大量数据，想移交所有权，但需要确保在操作时数据不会被复制
  // 3. 使用某个类型时，只关心它是否实现了某个 trait，而不关心它的具体类型

  // 将 5 存储在堆上，当 b 走出作用域时，会自动清理栈上的指针和堆里存放的数据
  let b = Box::new(5);
  println!("{}", b);

  // 使用 Box 赋能递归类型
  // 在编译时，Rust 需要知道一个类型所占的大小空间，而递归类型的大小无法在编译时确定
  // 下面的代码会报错，因为无法知道递归类型的大小
  // enum List {
  //   Cons(i32, List),
  //   Nil,
  // }

  // let _list = List::Cons(1, List::Cons(2, List::Cons(3, List::Nil)));

  // Rust 如何确定一个枚举类型所占的空间大小呢？
  // 它会遍历所有的变体，然后找到需要最大存储空间的变体，因为每个枚举只能同时存在一个变体
  // 以下面的 _Message 枚举为例，Quit 变体不需要占用空间，Move 变体则需要存储两个 i32 值的空间
  enum _Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
  }

  // Box<T> 是一个指针，不随其指向的数据的大小变化而变化，因此 Rust 总是知道它需要的空间大小
  // 因而我们可以通过 Box<T> 这种“间接”存储的能力改写上面 List 枚举的代码：
  enum List {
    Cons(i32, Box<List>),
    Nil,
  }

  let _list = List::Cons(
    1,
    Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
  );

  // Deref trait
  // 实现 Deref trait 可以自定义解引用符号 * 的行为，智能指针可以像常规引用一样处理
  struct MyBox<T>(T);

  impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
      MyBox(x)
    }
  }

  impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
      &self.0
    }
  }

  let x = 5;
  let y = MyBox::new(x);

  // 任何实现了 Deref trait 的类型，调用 * 的时候，相当于先执行 deref 方法
  // 再执行 *，对于编译器来讲，实际上 * 的作用只有一种，就是取得引用所指向的值
  assert_eq!(x, *y); // *(y.deref())

  // 隐式解引用转换 Deref Coercion
  // 一种为函数和方法提供的便捷特性
  // 如果一个类型实现了 Deref trait，这种特性会将其转换为经过 deref 操作后生成的引用
  // 当把某个类型的引用传递给函数或方法时，如果类型不匹配，隐式解引用转换就会发生
  // 编译器会对 deref 进行一系列调用，把它转换为所需的参数类型
  // 在编译时完成，因此没有额外的性能开销

  fn greeting(name: &str) {
    println!("Good morning, {}!", name);
  }

  let name = MyBox(String::from("Tyrion Lannister"));

  // 这里就发生了隐式解引用转换
  // &m 的类型为 &MyBox<String>，deref 后为 &String
  // 而标准库中的 String 也实现了 Deref trait，它会返回 &str
  // 也就是一路通过 deref 解引用，知道符合参数类型为止
  // 如果 Rust 没有这个特性，那么下面的代码应该这样写：greeting(&(*name)[..]);
  greeting(&name);

  // 解引用与可变性
  // 可以使用 DerefMut trait 重载可变引用的 * 运算符
  // 下列三种情况会发生隐式解引用转换：
  // 1. 当 T:Deref<Target=U> 时，允许 &T 转换为 &U
  // 2. 当 T:DerefMut<Target=U> 时，允许 &mut T 转换为 &mut U
  // 3. 当 T:Deref<Target=U> 时，允许 &mut T 转换为 &U

  // Drop trait
  // 实现 Drop trait，可以让我们自定义当值将要离开作用域时发生的动作
  // 常见有文件、网络资源释放等，任何类型都可以实现 Drop trait
  // 只需要实现一个 drop 方法，prelude 模块
  struct Person {
    name: String,
  }
  impl Drop for Person {
    fn drop(&mut self) {
      println!("{} dropped", self.name);
    }
  }

  let _danny = Person {
    name: String::from("Danny"),
  };

  // Rust 不允许手动调用 drop 方法
  // 可以调用 std::mem::drop 方法来提前 drop 值，也在预导入模块中
  let v = String::from("xx");
  drop(v);

  // v 已经失效
  // println!("{}", v);

  // Rc<T> 引用计数智能指针
  // 有时一个值会有多个所有者
  // Rc<T> 内部维护了一个引用次数的计数器，当计数器为 0 时，说明引用可以被安全地清理掉了
  //
  // 使用场景
  // 当需要把堆上的数据给程序的多个部分使用时，但在编译时无法确定哪个部分最后使用完
  // 只能用于单线程

  // 下面的代码会报错，因为 list_a 的所有权已经转移
  // let list_a = List::Cons(5, Box::new(List::Cons(10, Box::new(List::Nil))));
  // let _list_b = List::Cons(3, Box::new(list_a));
  // let _list_c = List::Cons(4, Box::new(list_a));
}
