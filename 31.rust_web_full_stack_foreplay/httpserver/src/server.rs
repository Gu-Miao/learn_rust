use super::router::Router;
use http::http_request::HttpRequest;
use std::{io::Read, net::TcpListener};

pub struct Server<'a> {
  addr: &'a str,
}

impl<'a> Server<'a> {
  pub fn new(addr: &'a str) -> Server {
    Server { addr }
  }

  pub fn run(&self) {
    let listener = TcpListener::bind(self.addr).unwrap();
    let mut buffer = [0; 1024];

    for stream in listener.incoming() {
      let mut stream = stream.unwrap();

      stream.read(&mut buffer).unwrap();

      let req: HttpRequest = String::from_utf8(buffer.to_vec()).unwrap().into();
      println!("{:?} {:?} {:?}", req.method, req.version, req.resource);

      Router::route(req, &mut stream);
    }
  }
}
