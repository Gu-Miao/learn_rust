//! crate2
//!
//! some description of crate2

pub use cal::add_two;

pub mod cal {
  use crate1;

  /// add two to the number given
  ///
  /// # Examples
  ///
  /// ```
  /// let five = crate2::add_two(3);
  ///
  /// assert_eq!(five, 5);
  /// ```
  pub fn add_two(x: i32) -> i32 {
    crate1::add_one(crate1::add_one(x))
  }
}
