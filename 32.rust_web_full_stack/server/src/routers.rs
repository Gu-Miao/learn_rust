use crate::handlers::{course::*, general::*};
use actix_web::web;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
  cfg.route("/health", web::get().to(health_check_handler));
}

pub fn course_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("/courses")
      .route("/", web::post().to(create_course))
      .route("/{teacher_id}", web::get().to(get_courses_of_teacher))
      .route("/{teacher_id}/{course_id}", web::get().to(get_course))
      .route("/{teacher_id}/{course_id}", web::delete().to(remove_course))
      .route("/{teacher_id}/{course_id}", web::put().to(update_course)),
  );
}
