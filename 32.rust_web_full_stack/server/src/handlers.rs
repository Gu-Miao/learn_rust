use super::db_access::*;
use super::models::Course;
use super::state::AppState;
use actix_web::{web, HttpResponse};

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
  let health_check_response = &app_state.health_check_response;
  let mut visit_count = app_state.visit_count.lock().unwrap();
  let response = format!("{} {} times", health_check_response, visit_count);

  *visit_count += 1;
  HttpResponse::Ok().json(&response)
}

pub async fn new_course(
  new_course: web::Json<Course>,
  app_state: web::Data<AppState>,
) -> HttpResponse {
  let course = new_course_db(&app_state.db, new_course.into()).await;
  HttpResponse::Ok().json(course)
}

pub async fn get_courses_for_teacher(
  path: web::Path<i32>,
  app_state: web::Data<AppState>,
) -> HttpResponse {
  let courses = get_courses_for_teacher_db(&app_state.db, path.into_inner()).await;
  HttpResponse::Ok().json(courses)
}

pub async fn get_courses_detail(
  path: web::Path<(i32, i32)>,
  app_state: web::Data<AppState>,
) -> HttpResponse {
  let (teacher_id, id) = path.into_inner();
  let course = get_courses_detail_db(&app_state.db, id, teacher_id).await;
  HttpResponse::Ok().json(course)
}

#[cfg(test)]
mod tests {
  use super::*;
  use actix_web::http::StatusCode;
  use dotenvy::dotenv;
  use sqlx::postgres::PgPoolOptions;
  use std::env;
  use std::sync::Mutex;

  #[actix_rt::test]
  async fn post_course_test() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let db = PgPoolOptions::new().connect(&database_url).await.unwrap();

    let app_state = web::Data::new(AppState {
      health_check_response: "".to_string(),
      visit_count: Mutex::new(0),
      db,
    });
    let course = web::Json(Course {
      teacher_id: 0,
      name: "JavaScript ES6".to_string(),
      id: None,
      time: None,
    });

    let res = new_course(course, app_state).await;
    assert_eq!(res.status(), StatusCode::OK);
  }

  #[actix_rt::test]
  async fn test_get_courses_for_teacher() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let db = PgPoolOptions::new().connect(&database_url).await.unwrap();

    let app_state = web::Data::new(AppState {
      health_check_response: "".to_string(),
      visit_count: Mutex::new(0),
      db,
    });

    let res = get_courses_for_teacher(web::Path::from(0), app_state).await;
    assert_eq!(res.status(), StatusCode::OK);
  }

  #[actix_rt::test]
  async fn test_get_course_detail() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let db = PgPoolOptions::new().connect(&database_url).await.unwrap();

    let app_state = web::Data::new(AppState {
      health_check_response: "".to_string(),
      visit_count: Mutex::new(0),
      db,
    });

    let res = get_courses_detail(web::Path::from((0, 1)), app_state).await;
    assert_eq!(res.status(), StatusCode::OK);
  }
}
