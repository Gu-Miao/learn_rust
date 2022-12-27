use http::{http_request::*, http_response::*};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;

#[derive(Deserialize, Serialize)]
pub struct OrderStatus {
  order_id: i32,
  order_date: String,
  order_status: String,
}

pub trait Handler {
  fn handle(req: &HttpRequest) -> HttpResponse;
  fn load_file(file_name: &str) -> Option<String> {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    let path = format!("{}/{}", public_path, file_name);

    fs::read_to_string(path).ok()
  }
}

pub struct APIHandler;

impl APIHandler {
  pub fn load_json() -> Vec<OrderStatus> {
    let default_path = format!("{}/data", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    let path = format!("{}/{}", public_path, "orders.json");
    let content = fs::read_to_string(path).unwrap();
    let orders: Vec<OrderStatus> = serde_json::from_str(content.as_str()).unwrap();

    orders
  }
}

impl Handler for APIHandler {
  fn handle(req: &HttpRequest) -> HttpResponse {
    let Resource::Path(s) = &req.resource;
    let route: Vec<&str> = s.split("/").collect();

    match route[2] {
      "shipping" if route.len() > 3 && route[3] == "orders" => {
        let body = serde_json::to_string(&Self::load_json()).unwrap();
        let mut headers = HashMap::new();

        headers.insert("Content-Type", "application/json");

        HttpResponse::new("200", Some(headers), Some(body))
      }
      _ => HttpResponse::new("404", None, Self::load_file("404.html")),
    }
  }
}

pub struct StaticHandler;

impl Handler for StaticHandler {
  fn handle(req: &HttpRequest) -> HttpResponse {
    let Resource::Path(s) = &req.resource;
    let route: Vec<&str> = s.split("/").collect();

    match route[1] {
      "" => HttpResponse::new("200", None, Self::load_file("index.html")),
      path => match Self::load_file(path) {
        Some(content) => {
          let mut headers = HashMap::new();
          if path.ends_with(".css") {
            headers.insert("Content-Type", "text/css");
          } else if path.ends_with(".js") {
            headers.insert("Content-Type", "text/javascript");
          } else {
            headers.insert("Content-Type", "text/html");
          }
          HttpResponse::new("200", Some(headers), Some(content))
        }
        None => HttpResponse::new("404", None, Self::load_file("404.html")),
      },
    }
  }
}

pub struct NotFoundHandler;

impl Handler for NotFoundHandler {
  fn handle(_: &HttpRequest) -> HttpResponse<'static> {
    HttpResponse::new("404", None, Self::load_file("404.html"))
  }
}
