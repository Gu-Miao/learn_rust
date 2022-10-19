// 通常用于描述 crate 或模块

//! # crate1
//!
//! This chapter mainly introduces the usage and configuration of cargo

/// Adds one to the number given
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = crate1::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
  x + 1
}

// 其他常用章节
// Panics 描述程序可能恐慌的场景
// Errors 如果返回 Result，描述可能错误的情况
// Safety 如果函数处于 unsafe 调用，应解释 unsafe 的原因，以及调用者应确保的使用前提

// 运行 cargo test 的时候会对文档注释中的代码段进行测试

// 使用 pub use 重新导出以简化路径
pub use color::PrimitiveColor;

/// color
///
/// mod of colors
pub mod color {

  /// primitive color enum
  pub enum PrimitiveColor {
    Red,
    Green,
    Blue,
  }

  impl PrimitiveColor {
    /// get name of color
    ///
    /// # Example
    ///
    /// ```
    /// use crate1::PrimitiveColor;
    ///
    /// let color_name = PrimitiveColor::get_color_name(PrimitiveColor::Red);
    ///
    /// assert_eq!("red", color_name);
    /// ```
    pub fn get_color_name(color: PrimitiveColor) -> &'static str {
      match color {
        PrimitiveColor::Red => "red",
        PrimitiveColor::Green => "green",
        PrimitiveColor::Blue => "blue",
      }
    }
  }
}
