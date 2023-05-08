use std::future::Future;
use std::process::Command;
use actix_web::{HttpResponse, web, put, delete, get};
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
        .service(delete_all_submissions)
        .service(run_generate);
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

#[get("/api/run-generate")]
async fn run_generate(_: Data<DbPool>) -> HttpResponse {
    web::block(move || {
        Command::new("sh")
            .arg("-c")
            .arg("psql postgres://postgres:013551@localhost/infoarena -q -f ../../../data-generator/test.sql")
            .output()
            .expect("failed to execute process");
    }).await.unwrap();

    HttpResponse::Ok().finish()
}