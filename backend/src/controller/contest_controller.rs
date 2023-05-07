use actix_web::{HttpResponse, web, post, delete, put, get};
use actix_web::web::{Data, Json, Path, ReqData};
use actix_web_httpauth::middleware::HttpAuthentication;
use serde::Deserialize;
use crate::DbPool;
use crate::middleware::authentication_validator;
use crate::model::contest;
use crate::model::contest::{Contest, NewContest, UpdContest};
use crate::model::dto::contest_dto::{ContestDTO, ContestWithCreatorDTO};
use crate::model::dto::pagination_dto::PaginationDTO;
use crate::model::dto::token_claims::TokenClaims;
use crate::repository::{contest_repository, participates_repository, user_credentials_repo};

pub fn contest_config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_contest_autocomplete)
        .service(all_contests)
        .service(get_contest_by_id);
}

pub fn contest_restricted(cfg: &mut web::ServiceConfig) {
    cfg.service(add_contest)
        .service(delete_contest)
        .service(update_contest);
}

#[derive(Deserialize)]
struct Autocomplete {
    name: Option<String>
}

#[post("/api/contest")]
async fn add_contest(pool: Data<DbPool>, req_user: Option<ReqData<TokenClaims>>, mut new_contest: Json<NewContest>) -> HttpResponse {
    new_contest.uid = Some(req_user.unwrap().id);

    web::block(move || {
        let mut conn = pool.get().unwrap();
        contest_repository::add_contest(&mut conn, new_contest.into_inner());
    }).await.unwrap();
    HttpResponse::Ok().finish()
}

#[delete("/api/contest/{id}")]
async fn delete_contest(pool: Data<DbPool>, req_user: Option<ReqData<TokenClaims>>, path: Path<i32>) -> HttpResponse {
    let token_data = req_user.unwrap();
    let id = path.into_inner();
    match web::block(move || {
        let mut conn = pool.get().unwrap();
        let contest = contest_repository::get_contest_by_id(&mut conn, id).unwrap().unwrap();

        if token_data.role == "regular" && token_data.id != contest.uid {
            return Err(());
        }

        contest_repository::delete_contest(&mut conn, id);
        Ok(())
    }).await.unwrap() {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::Unauthorized().finish()
    }
}

#[put("/api/contest")]
async fn update_contest(pool: Data<DbPool>, req_user: Option<ReqData<TokenClaims>>, new_contest: Json<UpdContest>) -> HttpResponse {
    let token_data = req_user.unwrap();
    match web::block(move || {
        let mut conn = pool.get().unwrap();

        let contest = contest_repository::get_contest_by_id(&mut conn, new_contest.id).unwrap().unwrap();
        if token_data.role == "regular" && token_data.id != contest.uid {
            return Err(());
        }

        contest_repository::update_contest(&mut conn, new_contest.into_inner());
        Ok(())
    }).await.unwrap() {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::Unauthorized().finish()
    }
}

#[get("/api/contest")]
async fn all_contests(pool: Data<DbPool>, query: web::Query<PaginationDTO>) -> HttpResponse {
    let mut contests = web::block(move || {
        let mut conn = pool.get().unwrap();
        let contests = contest_repository::get_contests_paginated(&mut conn, query.into_inner()).unwrap();

        let mut res = vec![];
        for contest in contests {
            let cnt = participates_repository::get_participation_by_cid(&mut conn, contest.id).unwrap().len() as i32;
            let creator = user_credentials_repo::get_user_credentials_by_id(&mut conn, contest.uid).unwrap().username;
            res.push(ContestWithCreatorDTO{contest: contest.clone(), cnt, creator});
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