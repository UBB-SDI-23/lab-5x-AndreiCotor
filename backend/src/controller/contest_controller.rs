use actix_web::{HttpResponse, web, post, delete, put, get};
use actix_web::web::{Data, Json, Path};
use crate::DbPool;
use crate::model::contest::{Contest, NewContest};
use crate::repository::contest_repository;

pub fn contest_config(cfg: &mut web::ServiceConfig) {
    cfg.service(add_contest)
        .service(delete_contest)
        .service(update_contest)
        .service(all_contests)
        .service(get_contest_by_id);
}

#[post("/contest")]
async fn add_contest(pool: Data<DbPool>, new_contest: Json<NewContest>) -> HttpResponse {
    web::block(move || {
        let mut conn = pool.get().unwrap();
        contest_repository::add_contest(&mut conn, new_contest.into_inner());
    }).await.unwrap();
    HttpResponse::Ok().finish()
}

#[delete("/contest/{id}")]
async fn delete_contest(pool: Data<DbPool>, path: Path<i32>) -> HttpResponse {
    web::block(move || {
        let mut conn = pool.get().unwrap();
        contest_repository::delete_contest(&mut conn, path.into_inner());
    }).await.unwrap();

    HttpResponse::Ok().finish()
}

#[put("/contest")]
async fn update_contest(pool: Data<DbPool>, new_contest: Json<Contest>) -> HttpResponse {
    web::block(move || {
        let mut conn = pool.get().unwrap();
        contest_repository::update_contest(&mut conn, new_contest.into_inner());
    }).await.unwrap();

    HttpResponse::Ok().finish()
}

#[get("/contest")]
async fn all_contests(pool: Data<DbPool>) -> HttpResponse {
    let contests = web::block(move || {
        let mut conn = pool.get().unwrap();
        contest_repository::get_all_contests(&mut conn)
    }).await.unwrap().map_err(|_| HttpResponse::InternalServerError().finish()).unwrap();

    HttpResponse::Ok().json(contests)
}

#[get("/contest/{id}")]
async fn get_contest_by_id(pool: Data<DbPool>, path: Path<i32>) -> HttpResponse {
    let id = path.into_inner();

    let con = web::block(move || {
        let mut conn = pool.get().unwrap();
        contest_repository::get_contest_by_id(&mut conn, id)
    }).await.unwrap().map_err(|_| HttpResponse::InternalServerError().finish()).unwrap().unwrap();

    HttpResponse::Ok().json(con)
}