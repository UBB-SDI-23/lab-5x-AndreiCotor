use std::future::Future;
use actix_web::{HttpResponse, web, put, delete};
use actix_web::web::{Data, Json};
use diesel::QueryResult;
use crate::DbPool;
use crate::model::user_credentials::UpdateRoleCredentials;
use crate::repository::{contest_repository, participates_repository, problem_repository, submission_repository, user_credentials_repo};

pub fn admin_config(cfg: &mut web::ServiceConfig) {
    cfg.service(update_role)
        .service(delete_all_contests)
        .service(delete_all_participations)
        .service(delete_all_problems)
        .service(delete_all_submissions);
}

#[put("/api/update-role")]
async fn update_role(pool: Data<DbPool>, credential_json: Json<UpdateRoleCredentials>) -> HttpResponse {
    match web::block(move || {
        let mut conn = pool.get().unwrap();
        user_credentials_repo::update_user_credentials_role(&mut conn, credential_json.into_inner())
    }).await.unwrap() {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::BadRequest().finish()
    }
}

#[delete("/api/all-contests")]
async fn delete_all_contests(pool: Data<DbPool>) -> HttpResponse {
    match web::block(move || {
        let mut conn = pool.get().unwrap();
        contest_repository::delete_all_contests(&mut conn)
    }).await.unwrap() {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

#[delete("/api/all-participates")]
async fn delete_all_participations(pool: Data<DbPool>) -> HttpResponse {
    match web::block(move || {
        let mut conn = pool.get().unwrap();
        participates_repository::delete_all_participations(&mut conn)
    }).await.unwrap() {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

#[delete("/api/all-problems")]
async fn delete_all_problems(pool: Data<DbPool>) -> HttpResponse {
    match web::block(move || {
        let mut conn = pool.get().unwrap();
        problem_repository::delete_all_problems(&mut conn)
    }).await.unwrap() {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

#[delete("/api/all-submissions")]
async fn delete_all_submissions(pool: Data<DbPool>) -> HttpResponse {
    match web::block(move || {
        let mut conn = pool.get().unwrap();
        submission_repository::delete_all_submissions(&mut conn)
    }).await.unwrap() {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}