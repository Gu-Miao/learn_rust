// 使用 newtype 模式实现类型的安全和抽象
// newtype 模式的好处
// 1. 用来静态地保证各种值之间不会混淆并表明值的单位
// 2. 为类型的某些细节提供抽象能力
// 3. 通过轻量级的封装来隐藏内部实现细节

// 使用类型别名创建同义词
type KiloMeters = i32;

fn add_km() {
  let x = 1;
  let y: KiloMeters = 2;
  println!("{}", x + y);
}

// Never 类型
// 名为 ! 的类型，也成为空类型
// 在不返回的函数中充当返回类型
// 不返回值的函数也称之为发散函数
// 比如 panic continue 等就会返回 Never 类型
// Never 类型可以转换为其他类型，所以可以使用在 match 的分支中

// 动态大小和 Sized 类型
// Rust 需要在编译时确定为一个类型分配多少空间
// 动态大小的类型 (DST, Dynamically Sized Types) 的概念
// 使用只有在运行时才确定大小的值
// 比如 str，只有在运行时才能确定字符串的长度

// 下面代码无法正常工作
// Rust 在编译时确定某个值占用多大空间，同一类型所有值必须使用等量内存
// 可以使用 &str 类型解决，它保存 str 的地址和长度，且本身大小固定
// fn strs() {
//   let s1: str = "Hello";
//   let s2: str = "Hi World!";
// }

// 如何使用动态大小？
// 附带一些额外的元数据来存储动态信息的大小，使用动态大小类型的数据时总会把它的值
// 放到某种指针之后，比如 str 和 &str

// 每个 trait 都是一个动态大小的类型，可以通过名称对其进行引用
// 为了将 trait 用作 trait 对象，必须将它放置在某种指针之后
// 比如 &dyn Trait 或者 Box<dyn Trait>

// 注意 2018 之前类型 Box<dyn Trait> 可以写成 Box<Trait>
// 2021 后强制要求 Box<dyn Trait> 以区分类型和 trait

// Sized trait
// 为了处理动态大小的类型，Rust 提供了一个 Sized trait 来确定一个类型的大小是否已知
// - 编译时可计算出大小的类型会自动实现这一 trait
// - Rust 会为每一个泛型函数隐式地添加 Sized 约束
// 即：fn some_fn<T>(t: T) {} -> fn some_fn<T: Sized>(t: T) {}
// 默认情况下，泛型函数只能用于编译时已经知道大小的类型，但可以通过特殊语法解除限制

// ?Sized trait 约束
// 只能用于 Sized trait 上，表示 T 可能实现了 Sized，也可能没实现
// fn some_fn<T: ?Sized>(t: &T) {} 这时 T 的大小不一定是确定的，我们需要将其放在某种指针后，此处变成了 &T

fn main() {
  add_km();
}
