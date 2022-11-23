use http::{http_request::*, http_response::*};
use std::collections::HashMap;
use std::env;
use std::fmt::format;
use std::fs;

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

impl Handler for APIHandler {
  fn handle(req: &HttpRequest) -> HttpResponse {
    HttpResponse::default()
  }
}

pub struct StaticHandler;

impl StaticHandler {
  pub fn handle(req: &HttpRequest) -> HttpResponse {
    HttpResponse::default()
  }
}

pub struct NotFoundHandler;

impl NotFoundHandler {
  pub fn handle(req: &HttpRequest) -> HttpResponse {
    HttpResponse::new("404", None, None)
  }
}
