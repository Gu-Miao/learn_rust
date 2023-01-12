drop table if exists teacher;

create table teacher (
  id serial primary key,
  name varchar(30) not null,
  profile varchar(140)
);
