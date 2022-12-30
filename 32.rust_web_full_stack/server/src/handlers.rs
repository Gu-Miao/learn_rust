use super::models::Course;
use super::state::AppState;
use actix_web::{web, HttpResponse};
use chrono::Utc;

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
  let course_count = app_state.courses.lock().unwrap().len();

  let new_course = Course {
    teacher_id: new_course.teacher_id,
    id: Some(course_count),
    name: new_course.name.clone(),
    time: Some(Utc::now().naive_utc()),
  };

  app_state.courses.lock().unwrap().push(new_course.clone());

  HttpResponse::Ok().json(new_course)
}

pub async fn get_courses_for_teacher(
  path: web::Path<usize>,
  app_state: web::Data<AppState>,
) -> HttpResponse {
  let teacher_id = path.into_inner();

  let filterd_courses = app_state
    .courses
    .lock()
    .unwrap()
    .clone()
    .into_iter()
    .filter(|course| course.teacher_id == teacher_id)
    .collect::<Vec<Course>>();

  HttpResponse::Ok().json(filterd_courses)
}

pub async fn get_courses_detail(
  path: web::Path<(usize, usize)>,
  app_state: web::Data<AppState>,
) -> HttpResponse {
  let (teacher_id, course_id) = path.into_inner();

  let selected_course = app_state
    .courses
    .lock()
    .unwrap()
    .clone()
    .into_iter()
    .find(|course| course.teacher_id == teacher_id && course.id == Some(course_id))
    .ok_or("Not Found");

  match selected_course {
    Ok(course) => HttpResponse::Ok().json(course),
    Err(msg) => HttpResponse::NotFound().json(msg),
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use actix_web::http::StatusCode;
  use std::sync::Mutex;

  #[actix_rt::test]
  async fn post_course_test() {
    let course = web::Json(Course {
      teacher_id: 0,
      name: "JavaScript ES6".into(),
      id: None,
      time: None,
    });

    let app_state = web::Data::new(AppState {
      health_check_response: "".to_string(),
      visit_count: Mutex::new(0),
      courses: Mutex::new(vec![]),
    });

    let res = new_course(course, app_state).await;
    assert_eq!(res.status(), StatusCode::OK);
  }

  #[actix_rt::test]
  async fn test_get_courses_for_teacher() {
    let course = Course {
      teacher_id: 0,
      name: "JavaScript ES6".into(),
      id: Some(0),
      time: Some(Utc::now().naive_utc()),
    };
    let app_state = web::Data::new(AppState {
      health_check_response: "".to_string(),
      visit_count: Mutex::new(0),
      courses: Mutex::new(vec![course]),
    });

    let res = get_courses_for_teacher(web::Path::from(0), app_state).await;

    assert_eq!(res.status(), StatusCode::OK);
  }

  #[actix_rt::test]
  async fn test_get_course_detail() {
    let app_state = web::Data::new(AppState {
      health_check_response: "".to_string(),
      visit_count: Mutex::new(0),
      courses: Mutex::new(vec![]),
    });
    let res = get_courses_detail(web::Path::from((0, 0)), app_state).await;
    assert_eq!(res.status(), StatusCode::NOT_FOUND);
  }
}
