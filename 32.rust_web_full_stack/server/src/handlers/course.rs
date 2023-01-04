use crate::db_access::course::*;
use crate::error::MyError;
use crate::models::course::*;
use crate::state::AppState;
use actix_web::{web, HttpResponse};

pub async fn create_course(
  new_course: web::Json<CreateCourseDTO>,
  app_state: web::Data<AppState>,
) -> Result<HttpResponse, MyError> {
  db_create_course(&app_state.db, new_course.try_into()?)
    .await
    .map(|course| HttpResponse::Ok().json(course))
}

pub async fn get_courses_of_teacher(
  path: web::Path<i32>,
  app_state: web::Data<AppState>,
) -> Result<HttpResponse, MyError> {
  db_get_courses_of_teacher(&app_state.db, path.into_inner())
    .await
    .map(|courses| HttpResponse::Ok().json(courses))
}

pub async fn get_course(
  path: web::Path<(i32, i32)>,
  app_state: web::Data<AppState>,
) -> Result<HttpResponse, MyError> {
  let (teacher_id, id) = path.into_inner();
  db_get_course(&app_state.db, id, teacher_id)
    .await
    .map(|course| HttpResponse::Ok().json(course))
}

pub async fn remove_course(
  path: web::Path<(i32, i32)>,
  app_state: web::Data<AppState>,
) -> Result<HttpResponse, MyError> {
  let (teacher_id, id) = path.into_inner();
  db_remove_course(&app_state.db, teacher_id, id)
    .await
    .map(|msg| HttpResponse::Ok().json(msg))
}

pub async fn update_course(
  path: web::Path<(i32, i32)>,
  app_state: web::Data<AppState>,
  update_course: web::Json<UpdateCourseDTO>,
) -> Result<HttpResponse, MyError> {
  let (teacher_id, id) = path.into_inner();
  db_update_course(&app_state.db, teacher_id, id, update_course.try_into()?)
    .await
    .map(|course| HttpResponse::Ok().json(course))
}

#[cfg(test)]
mod tests {
  use super::*;
  use actix_web::http::StatusCode;
  use actix_web::ResponseError;
  use dotenvy::dotenv;
  use sqlx::postgres::PgPoolOptions;
  use std::env;
  use std::sync::Mutex;

  #[ignore]
  #[actix_rt::test]
  async fn test_create_course() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let db = PgPoolOptions::new().connect(&database_url).await.unwrap();

    let app_state = web::Data::new(AppState {
      health_check_response: "".to_string(),
      visit_count: Mutex::new(0),
      db,
    });
    let course = web::Json(CreateCourseDTO {
      teacher_id: 1,
      name: "JavaScript ES6".to_string(),
      description: Some("ECMAScript 6（简称ES6）是于2015年6月正式发布的JavaScript语言的标准，正式名为ECMAScript 2015（ES2015）。它的目标是使得JavaScript语言可以用来编写复杂的大型应用程序，成为企业级开发语言".into()),
      format: None,
      structure: None,
      duration: None,
      price: None,
      language: Some("JavaScript".into()),
      level: Some("初级".into()),
    });

    let res = create_course(course, app_state).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
  }

  #[actix_rt::test]
  async fn test_get_courses_of_teacher() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let db = PgPoolOptions::new().connect(&database_url).await.unwrap();

    let app_state = web::Data::new(AppState {
      health_check_response: "".to_string(),
      visit_count: Mutex::new(0),
      db,
    });

    let res = get_courses_of_teacher(web::Path::from(1), app_state)
      .await
      .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
  }

  #[actix_rt::test]
  async fn test_get_course_success() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let db = PgPoolOptions::new().connect(&database_url).await.unwrap();

    let app_state = web::Data::new(AppState {
      health_check_response: "".to_string(),
      visit_count: Mutex::new(0),
      db,
    });

    let res = get_course(web::Path::from((1, 1)), app_state)
      .await
      .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
  }

  #[actix_rt::test]
  async fn test_get_course_failure() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let db = PgPoolOptions::new().connect(&database_url).await.unwrap();

    let app_state = web::Data::new(AppState {
      health_check_response: "".to_string(),
      visit_count: Mutex::new(0),
      db,
    });

    let res = get_course(web::Path::from((9999, 9999)), app_state).await;

    match res {
      Ok(_) => println!("Something went wrong"),
      Err(err) => assert_eq!(err.status_code(), StatusCode::NOT_FOUND),
    };
  }

  #[actix_rt::test]
  async fn test_update_course_success() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let db = PgPoolOptions::new().connect(&database_url).await.unwrap();

    let app_state = web::Data::new(AppState {
      health_check_response: "".to_string(),
      visit_count: Mutex::new(0),
      db,
    });

    let update_course_dto = web::Json(UpdateCourseDTO {
      name: Some("Course changed".into()),
      description: None,
      format: None,
      structure: None,
      duration: None,
      price: None,
      language: None,
      level: None,
    });

    let res = update_course(web::Path::from((1, 1)), app_state, update_course_dto)
      .await
      .unwrap();

    assert_eq!(res.status(), StatusCode::OK)
  }

  #[ignore]
  #[actix_rt::test]
  async fn test_remove_course_success() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let db = PgPoolOptions::new().connect(&database_url).await.unwrap();

    let app_state = web::Data::new(AppState {
      health_check_response: "".to_string(),
      visit_count: Mutex::new(0),
      db,
    });

    let res = remove_course(web::Path::from((1, 1)), app_state)
      .await
      .unwrap();

    assert_eq!(res.status(), StatusCode::OK)
  }

  #[actix_rt::test]
  async fn test_remove_course_failure() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let db = PgPoolOptions::new().connect(&database_url).await.unwrap();

    let app_state = web::Data::new(AppState {
      health_check_response: "".to_string(),
      visit_count: Mutex::new(0),
      db,
    });

    let res = remove_course(web::Path::from((9999, 9999)), app_state).await;

    match res {
      Ok(_) => println!("Something went wrong"),
      Err(err) => assert_eq!(err.status_code(), StatusCode::NOT_FOUND),
    }
  }
}
