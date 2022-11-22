use std::collections::HashMap;
use std::io::{self, Write};

#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a> {
  version: &'a str,
  status_code: &'a str,
  status_text: &'a str,
  headers: Option<HashMap<&'a str, &'a str>>,
  msg_body: Option<String>,
}

impl<'a> Default for HttpResponse<'a> {
  fn default() -> Self {
    Self {
      version: "HTTP/1.1",
      status_code: "200",
      status_text: "OK",
      headers: None,
      msg_body: None,
    }
  }
}

impl<'a> From<HttpResponse<'a>> for String {
  fn from(res: HttpResponse) -> Self {
    let res = res.clone();
    format!(
      "{} {} {}\r\n{}Content-Length: {}\r\n\r\n{}",
      &res.version(),
      &res.status_code(),
      &res.status_text(),
      &res.headers(),
      &res.msg_body().len(),
      &res.msg_body()
    )
  }
}

impl<'a> HttpResponse<'a> {
  pub fn new(
    status_code: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    msg_body: Option<String>,
  ) -> Self {
    let mut res = HttpResponse::default();
    if status_code != "200" {
      res.status_code = status_code;
      res.status_text = match res.status_code {
        "400" => "Bad Request",
        "404" => "Not Found",
        "500" => "Internal Server Error",
        _ => "Not Found",
      };
    }
    res.headers = match headers {
      Some(_) => headers,
      _ => {
        let mut hash_map = HashMap::new();
        hash_map.insert("Content-type", "text/html");
        Some(hash_map)
      }
    };
    res.msg_body = msg_body;

    res
  }

  pub fn send_response(&self, write_stream: &mut impl Write) -> io::Result<()> {
    let res = self.clone();
    let res_str = String::from(res);
    write!(write_stream, "{}", res_str)
  }

  fn version(&self) -> &str {
    &self.version
  }

  fn status_code(&self) -> &str {
    &self.status_code
  }

  fn status_text(&self) -> &str {
    &self.status_text
  }

  fn headers(&self) -> String {
    let map = &self.headers.clone().unwrap();
    let mut header_str = "".into();
    for (k, v) in map.iter() {
      header_str = format!("{}{}: {}\r\n", header_str, k, v);
    }

    header_str
  }

  fn msg_body(&self) -> &str {
    match &self.msg_body {
      Some(msg) => msg.as_str(),
      None => "",
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_response_struct_creation_200() {
    let response_actual = HttpResponse::new("200", None, Some("xxxx".into()));
    let response_expected = HttpResponse {
      version: "HTTP/1.1",
      status_code: "200",
      status_text: "OK",
      headers: {
        let mut map = HashMap::new();
        map.insert("Content-type", "text/html");
        Some(map)
      },
      msg_body: Some("xxxx".into()),
    };
    assert_eq!(response_actual, response_expected);
  }

  #[test]
  fn test_response_struct_creation_404() {
    let response_actual = HttpResponse::new("404", None, Some("xxxx".into()));
    let response_expected = HttpResponse {
      version: "HTTP/1.1",
      status_code: "404",
      status_text: "Not Found",
      headers: {
        let mut map = HashMap::new();
        map.insert("Content-type", "text/html");
        Some(map)
      },
      msg_body: Some("xxxx".into()),
    };
    assert_eq!(response_actual, response_expected);
  }

  #[test]
  fn test_http_response_creation() {
    let response_expected = HttpResponse::new("404", None, Some("xxx".into()));
    let res_str: String = response_expected.into();
    let actual_str =
      "HTTP/1.1 404 Not Found\r\nContent-type: text/html\r\nContent-Length: 3\r\n\r\nxxx"
        .to_string();
    assert_eq!(actual_str, res_str);
  }
}
