use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::io;

fn general_routes(cfg: &mut web::ServiceConfig) {
  cfg.route("/", web::get().to(index_handler));
}

async fn index_handler() -> impl Responder {
  HttpResponse::Ok().json("Actix web is running!")
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
  let app = move || App::new().configure(general_routes);

  HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
