use std::{env, error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn error::Error>> {
  let content = fs::read_to_string(config.filename)?;
  let results = if config.case_sensitive {
    search(&config.query, &content)
  } else {
    search_case_insensitive(&config.query, &content)
  };
  for line in results {
    println!("{}", line);
  }
  Ok(())
}

pub struct Config {
  pub query: String,
  pub filename: String,
  pub case_sensitive: bool,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 3 {
      return Err("not enough arguments!");
    }

    let query = args[1].clone();
    let filename = args[2].clone();

    // 在 powershell 中，通过 $env:CASE_INSENSITIVE=1 设置环境变量，再执行
    // cargo run to poem.txt 测试
    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

    Ok(Config {
      query,
      filename,
      case_sensitive,
    })
  }
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
  let mut result = vec![];

  for line in content.lines() {
    if line.contains(query) {
      result.push(line);
    }
  }

  result
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
  let mut result = vec![];
  let query = query.to_lowercase();

  for line in content.lines() {
    if line.to_lowercase().contains(&query) {
      result.push(line);
    }
  }

  result
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn case_sensitive() {
    let query = "duct";
    let content = "\
Rust
safe, fast, productive.
Pick there.
Duct tape.";

    assert_eq!(vec!["safe, fast, productive."], search(query, content));
  }

  #[test]
  fn case_insensitive() {
    let query = "rUsT";
    let content = "\
Rust
safe, fast, productive.
Pick there.
Trust me.";

    assert_eq!(
      vec!["Rust", "Trust me."],
      search_case_insensitive(query, content)
    );
  }
}
