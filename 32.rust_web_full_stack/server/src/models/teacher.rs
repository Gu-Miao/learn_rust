use actix_web::web;
use serde::{Deserialize, Serialize};

use crate::error::MyError;

#[derive(Serialize, Debug)]
pub struct Teacher {
  pub id: i32,
  pub name: String,
  pub profile: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct CreateTeacherDTO {
  pub name: String,
  pub profile: Option<String>,
}

impl TryFrom<web::Json<CreateTeacherDTO>> for CreateTeacherDTO {
  type Error = MyError;

  fn try_from(dto: web::Json<CreateTeacherDTO>) -> Result<Self, Self::Error> {
    Ok(CreateTeacherDTO {
      name: dto.name.clone(),
      profile: dto.profile.clone(),
    })
  }
}

#[derive(Deserialize, Debug)]
pub struct UpdateTeacherDTO {
  pub name: Option<String>,
  pub profile: Option<String>,
}

impl TryFrom<web::Json<UpdateTeacherDTO>> for UpdateTeacherDTO {
  type Error = MyError;

  fn try_from(dto: web::Json<UpdateTeacherDTO>) -> Result<Self, Self::Error> {
    Ok(UpdateTeacherDTO {
      name: dto.name.clone(),
      profile: dto.profile.clone(),
    })
  }
}
