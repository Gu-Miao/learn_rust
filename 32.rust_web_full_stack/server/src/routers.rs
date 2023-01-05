use crate::handlers::{course::*, general::*, teacher::*};
use actix_web::web;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
  cfg.route("/health", web::get().to(health_check_handler));
}

pub fn course_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("/courses")
      .route("/", web::get().to(get_courses))
      .route("/", web::post().to(create_course))
      .route("/{id}", web::get().to(get_course))
      .route("/{id}", web::delete().to(remove_course))
      .route("/{id}", web::put().to(update_course)),
  );
}

pub fn teacher_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("/teachers")
      .route("/", web::get().to(get_teachers))
      .route("/", web::post().to(create_teacher))
      .route("/{id}", web::get().to(get_teacher))
      .route("/{id}", web::delete().to(remove_teacher))
      .route("/{id}", web::put().to(update_teacher)),
  );
}
