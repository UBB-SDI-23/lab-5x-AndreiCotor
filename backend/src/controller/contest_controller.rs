use actix_web::{HttpResponse, web, post, delete, put, get};
use actix_web::web::{Data, Json, Path};
use actix_web_httpauth::middleware::HttpAuthentication;
use serde::Deserialize;
use crate::DbPool;
use crate::middleware::authentication_validator;
use crate::model::contest;
use crate::model::contest::{Contest, NewContest};
use crate::model::dto::contest_dto::ContestDTO;
use crate::model::dto::pagination_dto::PaginationDTO;
use crate::repository::{contest_repository, participates_repository};

pub fn contest_config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_contest_autocomplete)
        .service(all_contests)
        .service(get_contest_by_id)
        .service(
        web::scope("")
            .wrap(HttpAuthentication::bearer(authentication_validator))
            .service(add_contest)
            .service(delete_contest)
            .service(update_contest), );
}

#[derive(Deserialize)]
struct Autocomplete {
    name: Option<String>
}

#[post("/api/contest")]
async fn add_contest(pool: Data<DbPool>, new_contest: Json<NewContest>) -> HttpResponse {
    web::block(move || {
        let mut conn = pool.get().unwrap();
        contest_repository::add_contest(&mut conn, new_contest.into_inner());
    }).await.unwrap();
    HttpResponse::Ok().finish()
}

#[delete("/api/contest/{id}")]
async fn delete_contest(pool: Data<DbPool>, path: Path<i32>) -> HttpResponse {
    web::block(move || {
        let mut conn = pool.get().unwrap();
        contest_repository::delete_contest(&mut conn, path.into_inner());
    }).await.unwrap();

    HttpResponse::Ok().finish()
}

#[put("/api/contest")]
async fn update_contest(pool: Data<DbPool>, new_contest: Json<Contest>) -> HttpResponse {
    web::block(move || {
        let mut conn = pool.get().unwrap();
        contest_repository::update_contest(&mut conn, new_contest.into_inner());
    }).await.unwrap();

    HttpResponse::Ok().finish()
}

#[get("/api/contest")]
async fn all_contests(pool: Data<DbPool>, query: web::Query<PaginationDTO>) -> HttpResponse {
    let mut contests = web::block(move || {
        let mut conn = pool.get().unwrap();
        let contests = contest_repository::get_contests_paginated(&mut conn, query.into_inner()).unwrap();

        let mut res = vec![];
        for contest in contests {
            let cnt = participates_repository::get_participation_by_cid(&mut conn, contest.id).unwrap().len() as i32;
            res.push(ContestDTO{contest: contest.clone(), cnt});
        }
        res
    }).await.map_err(|_| HttpResponse::InternalServerError().finish()).unwrap();

    contests.sort_by(|a, b| a.contest.id.cmp(&b.contest.id));

    HttpResponse::Ok().json(contests)
}

#[get("/api/contest/autocomplete")]
async fn get_contest_autocomplete(pool: Data<DbPool>, query: web::Query<Autocomplete>) -> HttpResponse {
    let mut contests = web::block(move || {
        let mut conn = pool.get().unwrap();
        contest_repository::get_contests_by_name(&mut conn, query.into_inner().name)
    }).await.unwrap().map_err(|_| HttpResponse::InternalServerError().finish()).unwrap();

    HttpResponse::Ok().json(contests)
}

#[get("/api/contest/{id}")]
async fn get_contest_by_id(pool: Data<DbPool>, path: Path<i32>) -> HttpResponse {
    let id = path.into_inner();

    let con = web::block(move || {
        let mut conn = pool.get().unwrap();
        contest_repository::get_contest_by_id(&mut conn, id)
    }).await.unwrap().map_err(|_| HttpResponse::InternalServerError().finish()).unwrap().unwrap();

    HttpResponse::Ok().json(con)
}