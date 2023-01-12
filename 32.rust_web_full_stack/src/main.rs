use actix_cors::Cors;
use actix_web::{http, web, App, HttpServer};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::io;
use std::sync::Mutex;

mod db_access;
mod error;
mod handlers;
mod models;
mod routers;
mod state;

use routers::*;
use state::AppState;

#[actix_rt::main]
async fn main() -> io::Result<()> {
  dotenv().ok();

  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
  let db = PgPoolOptions::new().connect(&database_url).await.unwrap();

  let shared_data = web::Data::new(AppState {
    health_check_response: "I'm OK".to_string(),
    visit_count: Mutex::new(0),
    db,
  });
  let app = move || {
    let cors = Cors::default()
      .allowed_origin_fn(|origin, _| origin.as_bytes().starts_with(b"http://localhost"))
      .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
      .allowed_headers(vec![
        http::header::AUTHORIZATION,
        http::header::ACCEPT,
        http::header::CONTENT_TYPE,
      ])
      .max_age(360);

    App::new()
      .app_data(shared_data.clone())
      .configure(general_routes)
      .configure(course_routes)
      .configure(teacher_routes)
      .wrap(cors)
  };

  HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
