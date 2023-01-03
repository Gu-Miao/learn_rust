use super::error::MyError;
use super::models::*;
use chrono::NaiveDateTime;
use sqlx::postgres::PgPool;

pub async fn get_courses_for_teacher_db(
  pool: &PgPool,
  teacher_id: i32,
) -> Result<Vec<Course>, MyError> {
  let record = sqlx::query!("select * from course where teacher_id = $1", teacher_id)
    .fetch_all(pool)
    .await?;

  let courses: Vec<Course> = record
    .iter()
    .map(|record| Course {
      id: Some(record.id),
      teacher_id: record.teacher_id,
      name: record.name.clone(),
      time: Some(NaiveDateTime::from(record.time.unwrap())),
    })
    .collect();

  match courses.len() {
    0 => Err(MyError::NotFound("Courses not found".into())),
    _ => Ok(courses),
  }
}

pub async fn get_courses_detail_db(
  pool: &PgPool,
  id: i32,
  teacher_id: i32,
) -> Result<Course, MyError> {
  let record = sqlx::query!(
    "select * from course where id = $1 and teacher_id = $2",
    id,
    teacher_id
  )
  .fetch_one(pool)
  .await?;

  Ok(Course {
    id: Some(record.id),
    teacher_id: record.teacher_id,
    name: record.name.clone(),
    time: Some(NaiveDateTime::from(record.time.unwrap())),
  })
}

pub async fn new_course_db(pool: &PgPool, new_course: Course) -> Result<Course, MyError> {
  let record = sqlx::query!(
    r#"insert into course (teacher_id, name)
     values ($1, $2)
     returning *"#,
    new_course.teacher_id,
    &new_course.name
  )
  .fetch_one(pool)
  .await?;

  Ok(Course {
    id: Some(record.id),
    teacher_id: record.teacher_id,
    name: record.name.clone(),
    time: Some(NaiveDateTime::from(record.time.unwrap())),
  })
}
