use single_web_server::ThreadPool;
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
  let listener = TcpListener::bind("0.0.0.0:7777").unwrap();
  let pool = ThreadPool::new(5);

  for stream in listener.incoming().take(3) {
    let stream = stream.unwrap();

    pool.execute(|| {
      handle_connection(stream);
    });
  }

  println!("Server is closing");
}

fn handle_connection(mut stream: TcpStream) {
  let mut buffer = [0; 1024];

  stream.read(&mut buffer).unwrap();

  // println!("Request: {}\n\n", String::from_utf8_lossy(&buffer[..]));

  let get = b"GET / HTTP/1.1\r\n";
  let content: String;
  let response: String;

  if buffer.starts_with(get) {
    content = fs::read_to_string("index.html").unwrap();
    response = format!("HTTP/1.1 200 OK\r\n\r\n{}", content);
  } else {
    content = fs::read_to_string("404.html").unwrap();
    response = format!("HTTP/1.1 404 NOT FOUND\r\n\r\n{}", content);
  }

  stream.write(response.as_bytes()).unwrap();
  stream.flush().unwrap();
}
