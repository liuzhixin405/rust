use crate::{db_access::{post_new_course_db, get_course_for_teacher_db, get_course_details_db}, errors::MyError};

use super::state::AppState;
use actix_web::{web,HttpResponse};
use super::models::Course;

pub async fn health_check_handler(app_state:web::Data<AppState>)->HttpResponse{
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times",health_check_response,visit_count);
    *visit_count +=1;
    HttpResponse::Ok().json(&response)
}
pub async fn new_course(new_course:web::Json<Course>,
    app_state:web::Data<AppState>,)-> Result<HttpResponse,MyError> {
  println!("Received new course");
  /* let course_count = app_state.courses
  .lock().unwrap().clone().into_iter().filter(|course|course.teacher_id== new_course.teacher_id)
  .collect::<Vec<Course>>().len();

  let new_course = Course{
    teacher_id:new_course.teacher_id,
    id:Some(course_count+1),
    name:new_course.name.clone(),
    time:Some(Utc::now().naive_utc()),
  };
  app_state.courses.lock().unwrap().push(new_course); */
  
  post_new_course_db(&app_state.db, new_course.into()).await.map(|course|  HttpResponse::Ok().json(course))

} 

pub async fn get_courses_for_teacher(app_state:web::Data<AppState>,params:web::Path<(usize,)>,)->Result<HttpResponse,MyError> {
    /* let teacher_id:usize = params.0;
    let filtered_courses = app_state.courses
    .lock()
    .unwrap()
    .clone()
    .into_iter()
    .filter(|course|course.teacher_id==teacher_id)
    .collect::<Vec<Course>>();

    if filtered_courses.len()>0{
        HttpResponse::Ok().json(filtered_courses)
    }else{
        HttpResponse::Ok().json("No course found for feacher".to_string())
    } */

    let teacher_id = i32::try_from(params.0).unwrap();
    get_course_for_teacher_db(&app_state.db, teacher_id).await
    .map(|courses|HttpResponse::Ok().json(courses))
    //HttpResponse::Ok().json(course)
}

pub async fn get_courses_detail(app_state:web::Data<AppState>,params:web::Path<(usize,usize)>,)->Result<HttpResponse,MyError>{
   /*  let(teacher_id,course_id) = params.0;
    let selected_course = app_state.courses
    .lock()
    .unwrap()
    .clone()
    .into_iter()
    .find(|x|x.teacher_id==teacher_id &&x.id==Some(course_id)
    ).ok_or("Course not found");
    if let Ok(course) = selected_course{
        HttpResponse::Ok().json(course)
    }else{
        HttpResponse::Ok().json("no data".to_string())
    } */
    let teacher_id = i32::try_from(params.0).unwrap();
    let course_id = i32::try_from(params.1).unwrap();
   get_course_details_db(&app_state.db, teacher_id,course_id).await.map(|course|  HttpResponse::Ok().json(course))
}
  

#[cfg(test)]
mod tests{
    use super::*;
    use actix_web::http::StatusCode;
    use std::sync::Mutex;
    //use chrono::NaiveDateTime;
    use dotenv::dotenv;
    use sqlx::postgres::PgPoolOptions;
    use std::env;

    #[actix_rt::test]
    async fn post_course_test(){
        dotenv().ok();
        let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL 没有在 .env文件里设置");
    
        let db_pool = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .unwrap();


        let course = web::Json(Course{
            teacher_id:1,
            name:"Test course".into(),
            id:Some(5),//每次测试需要更新一下，否则数据库重复
            time:None,
        });
        let app_state:web::Data<AppState>=web::Data::new(AppState{
            health_check_response:"".to_string(),
            visit_count:Mutex::new(0),
            //courses:Mutex::new(vec![]),
            db:db_pool,
        });

        let resp = new_course(course, app_state).await.unwrap();
        assert_eq!(resp.status(),StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_all_courses_success(){
        dotenv().ok();
        let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL 没有在 .env文件里设置");
    
        let db_pool = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .unwrap();
        let app_state:web::Data<AppState>=web::Data::new(AppState{
            health_check_response:"".to_string(),
            visit_count:Mutex::new(0),
            //courses:Mutex::new(vec![]),
            db:db_pool,
        });
        let teacher_id=web::Path::from((1,));

        let resp = get_courses_for_teacher(app_state, teacher_id).await.unwrap();
        assert_eq!(resp.status(),StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_one_course_success(){
        dotenv().ok();
        let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL 没有在 .env文件里设置");
    
        let db_pool = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .unwrap();
        let app_state:web::Data<AppState>=web::Data::new(AppState{
            health_check_response:"".to_string(),
            visit_count:Mutex::new(0),
            //courses:Mutex::new(vec![]),
            db:db_pool,
        });
        let params:web::Path<(usize,usize)>=web::Path::from((1,1));

        let resp = get_courses_detail(app_state, params).await.unwrap();
        assert_eq!(resp.status(),StatusCode::OK);
    }
}
