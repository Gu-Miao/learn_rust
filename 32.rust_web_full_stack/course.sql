drop table if exists course;

create table course (
  id serial primary key,
  teacher_id int not null,
  name varchar(140) not null,
  time timestamp default now(),
  description varchar(2000),
  format varchar(30),
  structure varchar(200),
  duration varchar(30),
  price int,
  language varchar(30),
  level varchar(30)
);
