use crate::error::MyError;
use crate::models::teacher::*;
use sqlx::postgres::PgPool;

pub async fn db_create_teacher(
  pool: &PgPool,
  create_teacher_dto: CreateTeacherDTO,
) -> Result<Teacher, MyError> {
  let record = sqlx::query_as!(
    Teacher,
    r#"
  insert into teacher (name, profile)
  values ($1, $2)
  returning *"#,
    create_teacher_dto.name,
    create_teacher_dto.profile
  )
  .fetch_one(pool)
  .await?;

  Ok(record)
}

pub async fn db_get_teachers(pool: &PgPool) -> Result<Vec<Teacher>, MyError> {
  let record = sqlx::query_as!(Teacher, "select * from teacher",)
    .fetch_all(pool)
    .await?;

  Ok(record)
}

pub async fn db_get_teacher(pool: &PgPool, id: i32) -> Result<Teacher, MyError> {
  let record = sqlx::query_as!(Teacher, "select * from teacher where id = $1", id)
    .fetch_optional(pool)
    .await?;

  match record {
    Some(teacher) => Ok(teacher),
    None => Err(MyError::NotFound("Teacher not found".into())),
  }
}

pub async fn db_remove_teacher(pool: &PgPool, id: i32) -> Result<String, MyError> {
  let record = sqlx::query!("delete from teacher where id = $1", id)
    .execute(pool)
    .await?;

  Ok(format!("Delete {:?} teacher", record))
}

pub async fn db_update_teacher(
  pool: &PgPool,
  id: i32,
  dto: UpdateTeacherDTO,
) -> Result<Teacher, MyError> {
  sqlx::query_as!(Teacher, "select * from teacher where id = $1", id)
    .fetch_one(pool)
    .await
    .map_err(|_| MyError::NotFound("Teacher not found".into()))?;

  let record = sqlx::query_as!(
    Teacher,
    r#"update teacher set name = $1, profile = $2
     where id = $3
     returning *"#,
    dto.name,
    dto.profile,
    id
  )
  .fetch_one(pool)
  .await?;

  Ok(record)
}
