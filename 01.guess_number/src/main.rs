use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
  println!("Welcom to guees number game!");

  // 生成随机数
  let secret_number = rand::thread_rng().gen_range(0, 100);

  loop {
    let mut guess = String::new();

    // 读取命令行输入
    // io::Result 是一个枚举，有 Ok 和 Err 两个变体
    io::stdin()
      .read_line(&mut guess)
      .expect("Failed to read line");

    // 将字符串转为数字
    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => {
        println!("Please input number!\n");
        continue;
      }
    };

    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal => {
        println!("You win!");
        break;
      }
    }
  }
}
