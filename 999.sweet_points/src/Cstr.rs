// std::ffi::CStr 是 Rust 中一个用于解包复制构造函数参数并获取其值的类型。
// 它允许你在不暴露原始 C 字符串的情况下获取其值。这在需要在 Rust 中调用 C
// 函数并传递字符串参数的场景下非常有用，因为在 Rust 中，你不能直接操作原始 C
// 字符串，但你可以创建一个新的 CStr 对象来模拟它。

// 下面是一个使用 std::ffi::CStr 的示例：

#[no_mangle]
pub extern "C" fn hello_world(str: *const c_char) -> i32 {
  unsafe {
    std::str_to_bytes(std::ptr::null_mut(), str as *const c_char)
      .unwrap_or(std::ptr::null())
      .len() as i32
  }
}

#[no_mangle]
pub extern "C" fn main() {
  let hello = "Hello, world!";
  let cstr = std::ffi::CStr::from_bytes(hello.as_bytes());
  println!("{}", cstr.to_string());
}

// 在上面的示例中，我们定义了一个 C 字符串 hello_world()，并将它作为 Rust 函数
//  hello_world() 的参数传递。我们使用 std::ffi::CStr::from_bytes() 函数将
//  C 字符串解包为一个 CStr 对象，并使用 cstr.to_string() 将其转换为字符串。

// 使用 std::ffi::CStr 的场景非常广泛，例如在 Rust 中调用 C 函数并传递字符串参
// 数时，你可以使用它来避免手动管理字符串的副本，并提高代码的可读性和可维护性。
