use super::handler::{APIHandler, Handler, NotFoundHandler, StaticHandler};
use http::http_request::*;
use std::net::TcpStream;

pub struct Router {}

impl Router {
  pub fn route(req: HttpRequest, stream: &mut TcpStream) {
    match req.method {
      Method::Get => match &req.resource {
        Resource::Path(s) => {
          let route: Vec<&str> = s.split("/").collect();
          match route[1] {
            "api" => {
              let res = APIHandler::handle(&req);
              let _ = res.send_response(stream);
            }
            _ => {
              let res = StaticHandler::handle(&req);
              let _ = res.send_response(stream);
            }
          }
        }
      },
      _ => {
        let res = NotFoundHandler::handle(&req);
        let _ = res.send_response(stream);
      }
    }
  }
}
