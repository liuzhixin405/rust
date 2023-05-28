use crate::models::teacher::*;
use crate::state::AppState;
use crate::errors::MyError;
use actix_web::{web,HttpResponse};
use crate::dbaccess::teacher::*;

pub async fn get_all_teachers(app_state:web::Data<AppState>)->Result<HttpResponse,MyError>{
    get_all_teachers_db(&app_state.db).await.map(|teachers|HttpResponse::Ok().json(teachers))
}

pub async fn get_teacher_details(app_state:web::Data<AppState>,params:web::Path<i32>)->Result<HttpResponse,MyError>{
    let teacher_id = params.into_inner();
    get_teacher_details_db(&app_state.db,teacher_id).await.map(|teacher|HttpResponse::Ok().json(teacher))
}
pub async fn post_new_teacher(new_teacher:web::Json<CreateTeacher>,app_state:web::Data<AppState>)->Result<HttpResponse,MyError>{
    post_new_teacher_db(&app_state.db,CreateTeacher::from(new_teacher)).await.map(|teacher|HttpResponse::Ok().json(teacher))
}

pub async fn update_teacher_details(app_state:web::Data<AppState>,params:web::Path<i32>,update_teacher:web::Json<UpdateTeacher>)->Result<HttpResponse,MyError>{
    let teacher_id = params.into_inner();
    update_teacher_details_db(&app_state.db,teacher_id,UpdateTeacher::from(update_teacher)).await.map(|teacher|HttpResponse::Ok().json(teacher))
}
pub async fn delete_teacher(app_state:web::Data<AppState>,params:web::Path<i32>)->Result<HttpResponse,MyError>{
    let teacher_id = params.into_inner();
    delete_teacher_db(&app_state.db,teacher_id).await.map(|teacher|HttpResponse::Ok().json(teacher))
}
#[cfg(test)]
mod tests{
    use super::*;
    use actix_web::http::StatusCode;
    use dotenv::dotenv;
    use sqlx::postgres::PgPoolOptions;
    use std::env;
    use std::sync::Mutex;

    #[actix_rt::test]
    async fn get_all_teachers_success_test(){
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("Database url not set");
        let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url).await.unwrap();
        let app_state = web::Data::new(AppState{
            db:pool,
            health_check_response:"".to_string(),
            visit_count:Mutex::new(0),
        });
        let resp = get_all_teachers(app_state).await.unwrap();
        assert_eq!(resp.status(),StatusCode::OK);
    }
    #[actix_rt::test]
    async fn get_tutor_detail_success_test(){
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("Database url not set");
        let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url).await.unwrap();
        let app_state = web::Data::new(AppState{
            db:pool,
            health_check_response:"".to_string(),
            visit_count:Mutex::new(0),
        });
        let params:web::Path<i32> = web::Path::from(1);
        let resp = get_teacher_details(app_state, params).await.unwrap();
        assert_eq!(resp.status(),StatusCode::OK);
    }
    #[actix_rt::test]
    async fn post_teacher_success_test(){
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("Database url not set");
        let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url).await.unwrap();
        let app_state = web::Data::new(AppState{
            db:pool,
            health_check_response:"".to_string(),
            visit_count:Mutex::new(0),
        });
        let new_teacher =CreateTeacher{
            name:"test".into(),
            picture_url:"http://test.com".into(),
            profile:"A teach;
            er in machine learning".into(),
        };
        let teacher_param = web::Json(new_teacher);
        let resp = post_new_teacher(teacher_param, app_state).await.unwrap();
        assert_eq!(resp.status(),StatusCode::OK);
    }
    #[actix_rt::test]
    async fn delete_teacher_success_test(){
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("Database url not set");
        let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url).await.unwrap();
        let app_state = web::Data::new(AppState{
            db:pool,
            health_check_response:"".to_string(),
            visit_count:Mutex::new(0),
        });
        let params:web::Path<i32> = web::Path::from(1);
        let resp = delete_teacher(app_state, params).await.unwrap();
        assert_eq!(resp.status(),StatusCode::OK);
    }
}