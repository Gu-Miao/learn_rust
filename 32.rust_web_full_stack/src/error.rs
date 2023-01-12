use actix_web::{error, http::StatusCode, HttpResponse};
use serde::Serialize;
use sqlx::error::Error as SqlxError;
use std::fmt;

#[derive(Debug, Serialize)]
pub struct MyErrorResponse {
  error_message: String,
}

#[derive(Debug, Serialize)]
pub enum MyError {
  DBError(String),
  ActixError(String),
  NotFound(String),
}

impl MyError {
  fn error_response(&self) -> String {
    match self {
      MyError::DBError(msg) => {
        println!("Database error occured: {}", msg);
        "Database Error".into()
      }
      MyError::ActixError(msg) => {
        println!("Server error occured: {}", msg);
        "Internal Server Error".into()
      }
      MyError::NotFound(msg) => {
        println!("Not found error occured: {}", msg);
        msg.into()
      }
    }
  }
}

impl error::ResponseError for MyError {
  fn status_code(&self) -> StatusCode {
    match self {
      MyError::DBError(_) | MyError::ActixError(_) => StatusCode::INTERNAL_SERVER_ERROR,
      MyError::NotFound(_) => StatusCode::NOT_FOUND,
    }
  }
  fn error_response(&self) -> HttpResponse {
    HttpResponse::build(self.status_code()).json(MyErrorResponse {
      error_message: self.error_response(),
    })
  }
}

impl fmt::Display for MyError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self)
  }
}

impl From<error::Error> for MyError {
  fn from(err: error::Error) -> Self {
    MyError::ActixError(err.to_string())
  }
}

impl From<SqlxError> for MyError {
  fn from(err: SqlxError) -> Self {
    MyError::DBError(err.to_string())
  }
}
