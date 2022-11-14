// 代码组织
// 哪些细节可以暴露，哪些是私有的
// 作用域内哪些名称有效

// Package (包)
// cargo 的特性，让你构建、测试、共享 crate
// 包含一个 Cargo.toml，他描述如何构建 Crates
// 只能包含 0-1 个 libarary crate
// 可以有任意数量的 binary crate
// 至少包含一个 crate

// Crate (单元包)
// 一个模块树，可以产生一个 libarary 或可执行文件
// src/main.rs: binary crate 的 root crate，crate 名与 package 名相同
// src/lib.rs: library crate 的 root crate，crate 名与 package 名相同
// src/bin 目录下的每个 crate 都是一个单独的 binary crate

// Module (模块)
// 在一个 crate 内，将代码分组方便复用，增加可读性
// 通过 mod 关键字创建 module，并且可以嵌套
mod father_mod {
  pub mod son {
    // Rust 中所有条目默认函数、方法、结构体、枚举、模块、常量等都是私有的
    // pub 关键字将 son 模块设为公共
    pub fn son_fn() {}
  }
}

// Path (路径)
// 绝对路径：从 crate root 开始找，使用 crate 名或者 crate 字面值
// 相对路径：从当前模块开始找，使用 self, super 或者当前模块标识符
fn _path_fn() {
  crate::father_mod::son::son_fn(); // 绝对路径
  father_mod::son::son_fn(); // 相对路径
}

mod fff {
  pub mod ff {
    pub mod f {
      pub fn _f_fn() {
        // 使用 super 关键字访问父级
        super::super::_fff_fn();
      }
    }
  }

  fn _fff_fn() {}

  pub struct _S {
    // 结构体中每个字段默认是私有的，需要使用 pub 关键字
    // 指定为公有
    pub a: u32,
    b: u8,
  }

  // 公共枚举的每个变体都是公共的
  pub enum _E {
    A,
    B,
  }
}

// 使用 use 关键字引入 son 模块
// 引入后仍然需要遵守私有性规则
use father_mod::son;

// 使用 as 创建别名
// use 前也可以使用 pub 关键字将模块导出
pub use father_mod::son as father_son;

// 使用外部包需要先在 Cargo.toml 写入以来版本，再使用 use 引入
// 标准库也被视为外部包
use rand::Rng;

// 使用嵌套路径清理大量 use 语句
// 可以使用 self 表示本身
// 如果在 JavaScript 中，可以表示为 import { default as io, Write } from 'std/io'
// use std::io::{self, Write};

// 将模块放于其他文件中
mod test_a;

pub fn main() {
  // 已经导入了 son 模块，直接使用其中的函数
  son::son_fn();

  // 使用别名
  father_son::son_fn();

  // 我们曾在猜数游戏中使用过 rand 包
  let _x = rand::thread_rng().gen_range(0, 100);
}
