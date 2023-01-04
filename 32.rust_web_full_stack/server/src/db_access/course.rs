use crate::error::MyError;
use crate::models::course::*;
use sqlx::postgres::PgPool;

pub async fn db_get_courses_of_teacher(
  pool: &PgPool,
  teacher_id: i32,
) -> Result<Vec<Course>, MyError> {
  let record = sqlx::query_as!(
    Course,
    "select * from course where teacher_id = $1",
    teacher_id
  )
  .fetch_all(pool)
  .await?;

  Ok(record)
}

pub async fn db_get_course(pool: &PgPool, id: i32, teacher_id: i32) -> Result<Course, MyError> {
  let record = sqlx::query_as!(
    Course,
    "select * from course where id = $1 and teacher_id = $2",
    id,
    teacher_id
  )
  .fetch_optional(pool)
  .await?;

  match record {
    Some(course) => Ok(course),
    None => Err(MyError::NotFound("Course not found".into())),
  }
}

pub async fn db_create_course(
  pool: &PgPool,
  new_course: CreateCourseDTO,
) -> Result<Course, MyError> {
  let record = sqlx::query_as!(
    Course,
    r#"insert into course 
     (teacher_id, name, description, format, structure, duration, price, language, level)
     values ($1, $2, $3, $4, $5, $6, $7, $8, $9)
     returning *"#,
    new_course.teacher_id,
    new_course.name,
    new_course.description,
    new_course.format,
    new_course.structure,
    new_course.duration,
    new_course.price,
    new_course.language,
    new_course.level
  )
  .fetch_one(pool)
  .await?;

  Ok(record)
}

pub async fn db_remove_course(pool: &PgPool, teacher_id: i32, id: i32) -> Result<String, MyError> {
  let record = sqlx::query!(
    "delete from course where teacher_id = $1 and id = $2",
    teacher_id,
    id
  )
  .execute(pool)
  .await?;

  Ok(format!("Delete {:?} courese", record))
}

pub async fn db_update_course(
  pool: &PgPool,
  teacher_id: i32,
  id: i32,
  dto: UpdateCourseDTO,
) -> Result<Course, MyError> {
  sqlx::query_as!(
    Course,
    "select * from course where teacher_id = $1 and id = $2",
    teacher_id,
    id
  )
  .fetch_one(pool)
  .await
  .map_err(|_| MyError::NotFound("Course not found".into()))?;

  let record = sqlx::query_as!(
    Course,
    r#"update course set name = $1, description = $2, format = $3,
     structure = $4, duration = $5, price = $6, language = $7, level = $8
     where teacher_id = $9 and id = $10
     returning *"#,
    dto.name,
    dto.description,
    dto.format,
    dto.structure,
    dto.duration,
    dto.price,
    dto.language,
    dto.level,
    teacher_id,
    id
  )
  .fetch_one(pool)
  .await?;

  Ok(record)
}
