use crate::error::MyError;
use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::convert::TryFrom;

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct Course {
  pub teacher_id: i32,
  pub id: i32,
  pub name: String,
  pub time: Option<NaiveDateTime>,
  pub description: Option<String>,
  pub format: Option<String>,
  pub structure: Option<String>,
  pub duration: Option<String>,
  pub price: Option<i32>,
  pub language: Option<String>,
  pub level: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CreateCourseDTO {
  pub teacher_id: i32,
  pub name: String,
  pub description: Option<String>,
  pub format: Option<String>,
  pub structure: Option<String>,
  pub duration: Option<String>,
  pub price: Option<i32>,
  pub language: Option<String>,
  pub level: Option<String>,
}

impl TryFrom<web::Json<CreateCourseDTO>> for CreateCourseDTO {
  type Error = MyError;

  fn try_from(course: web::Json<CreateCourseDTO>) -> Result<Self, Self::Error> {
    Ok(CreateCourseDTO {
      teacher_id: course.teacher_id,
      name: course.name.clone(),
      description: course.description.clone(),
      format: course.format.clone(),
      structure: course.structure.clone(),
      duration: course.duration.clone(),
      price: course.price,
      language: course.language.clone(),
      level: course.level.clone(),
    })
  }
}

#[derive(Deserialize, Debug, Clone)]
pub struct UpdateCourseDTO {
  pub name: Option<String>,
  pub description: Option<String>,
  pub format: Option<String>,
  pub structure: Option<String>,
  pub duration: Option<String>,
  pub price: Option<i32>,
  pub language: Option<String>,
  pub level: Option<String>,
}

impl TryFrom<web::Json<UpdateCourseDTO>> for UpdateCourseDTO {
  type Error = MyError;

  fn try_from(course: web::Json<UpdateCourseDTO>) -> Result<Self, Self::Error> {
    Ok(UpdateCourseDTO {
      name: course.name.clone(),
      description: course.description.clone(),
      format: course.format.clone(),
      structure: course.structure.clone(),
      duration: course.duration.clone(),
      price: course.price,
      language: course.language.clone(),
      level: course.level.clone(),
    })
  }
}
