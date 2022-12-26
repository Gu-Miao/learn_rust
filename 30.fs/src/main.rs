use std::fs::{self, File};
use std::io::{BufRead, BufReader, Read, Write};

// Rust 中文件操作主要依赖于 std::fs::File 这个结构体
// File 的所有方法都会返回一个 Result 枚举

// 打开文件
// File::open 函数用于以只读模式打开一个已经存在的文件并返回一个句柄；如果不存在，回抛出错误
fn open_a_txt() -> File {
  let handle = File::open("a.txt").unwrap();

  return handle;
}

// 创建文件
// File::create 函数以只写模式打开一个文件
// 如果文件存在则清空，否则创建新文件，并返回一个句柄
// 使用 fs::OpenOptions 可设置为追加模式，比如：fs::OpenOptions::new().append(true).open("a.txt")
fn create_a_txt() {
  let mut handle = File::create("a.txt").unwrap();

  handle
    .write_all("Hello Rust!\nI love Rust!\nLet's learn Rust!\n".as_bytes())
    .unwrap();
}

// 删除文件
// fs::remove_file 从文件系统中删除文件
fn remove_a_txt() {
  match fs::remove_file("a.txt") {
    Ok(_) => println!("a.txt has been removed\n"),
    Err(_) => (),
  }
}

// 文件内容的操作
// io::Read, io::Write
// 读取文件的三种方式
// Vec, String, 逐行读取
fn read_as_vec() {
  let mut handle = open_a_txt();
  let mut vec = Vec::new();

  // 执行完读取后，handle 中的数据被清空，如果再次读取 handle，为空
  handle.read_to_end(&mut vec).unwrap();

  println!("Vec: {:?}\n", vec);
}

fn read_as_string() {
  let mut handle = open_a_txt();
  let mut str = String::new();

  handle.read_to_string(&mut str).unwrap();

  println!("String: {}\n", str);
}

fn read_line_by_line() {
  let handle = open_a_txt();
  let reader = BufReader::new(handle);

  for (index, line) in reader.lines().enumerate() {
    let line = line.unwrap();
    println!("Line {}: {}", index + 1, line);
  }
}

fn write_a_txt() {
  let mut handle = fs::OpenOptions::new().append(true).open("a.txt").unwrap();

  handle
    .write("Oh ~ My god! Rust is hard!".as_bytes())
    .unwrap();
}

fn main() {
  remove_a_txt();
  create_a_txt();
  write_a_txt();
  read_as_vec();
  read_as_string();
  read_line_by_line();
}
