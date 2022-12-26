use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Method {
  Get,
  Post,
  Uninitialized,
}

impl From<&str> for Method {
  fn from(method: &str) -> Self {
    match method {
      "GET" => Method::Get,
      "POST" => Method::Post,
      _ => Method::Uninitialized,
    }
  }
}

#[derive(Debug, PartialEq)]
pub enum Version {
  V1_1,
  V2_0,
  Uninitialized,
}

impl From<&str> for Version {
  fn from(version: &str) -> Self {
    match version {
      "HTTP/1.1" => Version::V1_1,
      "HTTP/2.0" => Version::V2_0,
      _ => Version::Uninitialized,
    }
  }
}

#[derive(Debug, PartialEq)]
pub enum Resource {
  Path(String),
}

#[derive(Debug, PartialEq)]
pub struct HttpRequest {
  pub method: Method,
  pub version: Version,
  pub resource: Resource,
  pub headers: HashMap<String, String>,
  pub msg_body: String,
}

impl From<String> for HttpRequest {
  fn from(req: String) -> Self {
    let mut parsed_method = Method::Uninitialized;
    let mut parsed_version = Version::Uninitialized;
    let mut parsed_resource = Resource::Path("".to_string());
    let mut parsed_headers = HashMap::new();
    let mut parsed_msg_body = "";

    for line in req.lines() {
      if line.contains("HTTP") {
        let (method, resource, version) = process_req_line(line);
        parsed_method = method;
        parsed_resource = resource;
        parsed_version = version;
      } else if line.contains(":") {
        let (key, value) = process_req_header(line);
        parsed_headers.insert(key, value);
      } else if line.len() == 0 {
        // empty line
      } else {
        parsed_msg_body = line;
      }
    }

    HttpRequest {
      method: parsed_method,
      version: parsed_version,
      resource: parsed_resource,
      headers: parsed_headers,
      msg_body: parsed_msg_body.to_string(),
    }
  }
}

fn process_req_line(line: &str) -> (Method, Resource, Version) {
  let mut words = line.split_whitespace();
  let method = words.next().unwrap();
  let resource = words.next().unwrap();
  let version = words.next().unwrap();

  (
    method.into(),
    Resource::Path(resource.to_string()),
    version.into(),
  )
}

fn process_req_header(line: &str) -> (String, String) {
  let mut words = line.split(":");
  let mut key = String::from("");
  let mut value = String::from("");

  if let Some(k) = words.next() {
    key = k.to_string();
  };
  if let Some(v) = words.next() {
    value = v.to_string();
  };

  (key, value)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_method_into() {
    let method: Method = "GET".into();
    assert_eq!(method, Method::Get);
  }

  #[test]
  fn test_version_into() {
    let version: Version = "HTTP/1.1".into();
    assert_eq!(version, Version::V1_1);
  }

  #[test]
  fn test_read_http() {
    let req = String::from("GET /greeting HTTP/1.1\r\nAccept: */*\r\nAccept-Encoding: gzip, deflate, br\r\nAccept-Language: zh-CN,zh;q=0.9,en;q=0.8\r\nConnection: keep-alive\r\nHost: fanyi.baidu.com\r\nReferer: https://fanyi.baidu.com/\r\nsec-ch-ua: \"Google Chrome\";v=\"107\", \"Chromium\";v=\"107\", \"Not=A?Brand\";v=\"24\"\r\nsec-ch-ua-mobile: ?0\r\nsec-ch-ua-platform: \"Windows\"\r\nSec-Fetch-Dest: empty\r\nSec-Fetch-Mode: cors\r\nSec-Fetch-Site: same-origin\r\nUser-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/107.0.0.0 Safari/537.36\r\nHello World");
    let request: HttpRequest = req.into();

    assert_eq!(request.method, Method::Get);
    assert_eq!(request.resource, Resource::Path(String::from("/greeting")));
    assert_eq!(request.version, Version::V1_1);
    assert_eq!(
      &request
        .headers
        .get(&String::from("Accept-Language"))
        .unwrap()[..],
      " zh-CN,zh;q=0.9,en;q=0.8"
    );
    assert_eq!(request.msg_body, String::from("Hello World"));
  }
}
