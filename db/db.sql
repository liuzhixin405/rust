 drop Table if EXISTS course;
 CREATE Table course(
id serial primary key,
teacher_id int not null,
name varchar(140) not null,
tiem TIMESTAMP DEFAULT now()
 );

 insert into course(id,teacher_id,name,time)
 VALUES(1,1, 'First course','2023-01-01 09:10:00');

  insert into course(id,teacher_id,name,time)
 VALUES(2,1, 'Second course','2023-01-04 08:10:00');