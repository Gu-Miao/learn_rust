use crate::db_access::teacher::*;
use crate::error::MyError;
use crate::models::teacher::*;
use crate::state::AppState;
use actix_web::{web, HttpResponse};

pub async fn create_teacher(
  app_state: web::Data<AppState>,
  dto: web::Json<CreateTeacherDTO>,
) -> Result<HttpResponse, MyError> {
  db_create_teacher(&app_state.db, dto.try_into()?)
    .await
    .map(|teacher| HttpResponse::Ok().json(teacher))
}

pub async fn get_teachers(app_state: web::Data<AppState>) -> Result<HttpResponse, MyError> {
  db_get_teachers(&app_state.db)
    .await
    .map(|courses| HttpResponse::Ok().json(courses))
}

pub async fn get_teacher(
  app_state: web::Data<AppState>,
  path: web::Path<i32>,
) -> Result<HttpResponse, MyError> {
  let id = path.into_inner();
  db_get_teacher(&app_state.db, id)
    .await
    .map(|course| HttpResponse::Ok().json(course))
}

pub async fn remove_teacher(
  app_state: web::Data<AppState>,
  path: web::Path<i32>,
) -> Result<HttpResponse, MyError> {
  let id = path.into_inner();
  db_remove_teacher(&app_state.db, id)
    .await
    .map(|msg| HttpResponse::Ok().json(msg))
}

pub async fn update_teacher(
  app_state: web::Data<AppState>,
  path: web::Path<i32>,
  dto: web::Json<UpdateTeacherDTO>,
) -> Result<HttpResponse, MyError> {
  let id = path.into_inner();
  db_update_teacher(&app_state.db, id, dto.try_into()?)
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
  async fn test_create_teacher() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let db = PgPoolOptions::new().connect(&database_url).await.unwrap();

    let app_state = web::Data::new(AppState {
      health_check_response: "".to_string(),
      visit_count: Mutex::new(0),
      db,
    });
    let dto = web::Json(CreateTeacherDTO {
      name: "Wang Lei".to_string(),
      profile: Some("北京大学信息科学教授".into()),
    });

    let res = create_teacher(app_state, dto).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
  }

  #[actix_rt::test]
  async fn test_get_teachers() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let db = PgPoolOptions::new().connect(&database_url).await.unwrap();

    let app_state = web::Data::new(AppState {
      health_check_response: "".to_string(),
      visit_count: Mutex::new(0),
      db,
    });

    let res = get_teachers(app_state).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
  }

  #[actix_rt::test]
  async fn test_get_teacher_success() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let db = PgPoolOptions::new().connect(&database_url).await.unwrap();

    let app_state = web::Data::new(AppState {
      health_check_response: "".to_string(),
      visit_count: Mutex::new(0),
      db,
    });

    let res = get_teacher(app_state, web::Path::from(1)).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
  }

  #[actix_rt::test]
  async fn test_get_teacher_failure() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let db = PgPoolOptions::new().connect(&database_url).await.unwrap();

    let app_state = web::Data::new(AppState {
      health_check_response: "".to_string(),
      visit_count: Mutex::new(0),
      db,
    });

    let res = get_teacher(app_state, web::Path::from(9999)).await;

    match res {
      Ok(_) => println!("Something went wrong"),
      Err(err) => assert_eq!(err.status_code(), StatusCode::NOT_FOUND),
    };
  }

  #[actix_rt::test]
  async fn test_update_teacher_success() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let db = PgPoolOptions::new().connect(&database_url).await.unwrap();

    let app_state = web::Data::new(AppState {
      health_check_response: "".to_string(),
      visit_count: Mutex::new(0),
      db,
    });

    let dto = web::Json(UpdateTeacherDTO {
      name: Some("Wang TianLei".into()),
      profile: None,
    });

    let res = update_teacher(app_state, web::Path::from(1), dto)
      .await
      .unwrap();

    assert_eq!(res.status(), StatusCode::OK)
  }

  #[ignore]
  #[actix_rt::test]
  async fn test_remove_teacher_success() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let db = PgPoolOptions::new().connect(&database_url).await.unwrap();

    let app_state = web::Data::new(AppState {
      health_check_response: "".to_string(),
      visit_count: Mutex::new(0),
      db,
    });

    let res = remove_teacher(app_state, web::Path::from(1)).await.unwrap();

    assert_eq!(res.status(), StatusCode::OK)
  }

  #[actix_rt::test]
  async fn test_remove_teacher_failure() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let db = PgPoolOptions::new().connect(&database_url).await.unwrap();

    let app_state = web::Data::new(AppState {
      health_check_response: "".to_string(),
      visit_count: Mutex::new(0),
      db,
    });

    let res = remove_teacher(app_state, web::Path::from(9999)).await;

    match res {
      Ok(_) => println!("Something went wrong"),
      Err(err) => assert_eq!(err.status_code(), StatusCode::NOT_FOUND),
    }
  }
}
