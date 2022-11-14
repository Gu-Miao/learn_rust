use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
  let listener = TcpListener::bind("0.0.0.0:3000").unwrap();

  for stream in listener.incoming() {
    let mut stream = stream.unwrap();
    let mut buffer = [0; 2048];

    stream.read(&mut buffer).unwrap();
    println!("{:?}", stream);

    stream.write(&mut buffer).unwrap();
  }
}
