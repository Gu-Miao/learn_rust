// tests  子目录下的文件不会被视作 crates，而是被视普通的模块

pub fn helper() {
  println!("log from helper");
}
