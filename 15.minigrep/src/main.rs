use minigrep::{run, Config};
use std::{env, process};

fn main() {
  // 如果需要接受非法 unicode 字符，可以使用 env::args_os 方法，它会
  // 返回 OsString
  let args: Vec<String> = env::args().collect();

  let config = Config::new(&args).unwrap_or_else(|err| {
    // eprintln 宏会将内容打印到标准错误流中
    eprintln!("Problem parsing arguments: {}", err);
    process::exit(1);
  });

  if let Err(err) = run(config) {
    eprint!("Application error: {}", err);
    process::exit(1);
  };
}
