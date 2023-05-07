use actix_web::{HttpResponse, web, post, delete, put, get};
use actix_web::web::{Data, Json, Path, ReqData};
use actix_web_httpauth::middleware::HttpAuthentication;
use crate::DbPool;
use crate::middleware::authentication_validator;
use crate::model::dto::pagination_dto::ParticipationPaginationDTO;
use crate::model::dto::participates_dto::ParticipatesDTO;
use crate::model::dto::token_claims::TokenClaims;
use crate::model::participates::Participates;
use crate::repository::{contest_repository, participates_repository, users_repo};

pub fn participates_config(cfg: &mut web::ServiceConfig) {
    cfg.service(all_participates)
        .service(get_participates_by_id);
}

pub fn participates_restricted(cfg: &mut web::ServiceConfig) {
    cfg.service(add_participates)
        .service(delete_participates)
        .service(update_participates);
}

#[post("/api/participates")]
async fn add_participates(pool: Data<DbPool>, req_user: Option<ReqData<TokenClaims>>, participates_json: Json<Vec<Participates>>) -> HttpResponse {
    let token_data = req_user.unwrap();
    let participates_list = participates_json.into_inner();

    for participates in &participates_list {
        if !participates.is_valid() {
            return HttpResponse::BadRequest().finish();
        }
        if token_data.role == "regular" && participates.uid != token_data.id {
            return HttpResponse::Unauthorized().finish();
        }
    }

    let val = web::block(move || {
        let mut conn = pool.get().unwrap();
        participates_repository::add_multiple_participations(&mut conn, participates_list)
    }).await.unwrap();

    match val {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

#[delete("/api/participates/{id1}/{id2}")]
async fn delete_participates(pool: Data<DbPool>, req_user: Option<ReqData<TokenClaims>>, path: Path<(i32, i32)>) -> HttpResponse {
    let id = path.into_inner();
    let token_data = req_user.unwrap();

    if token_data.role == "regular" && id.0 != token_data.id {
        return HttpResponse::Unauthorized().finish();
    }

    let val = web::block(move || {
        let mut conn = pool.get().unwrap();
        participates_repository::delete_participation(&mut conn, id.0, id.1)
    }).await.unwrap();

    match val {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

#[put("/api/participates")]
async fn update_participates(pool: Data<DbPool>, req_user: Option<ReqData<TokenClaims>>, new_part: Json<Participates>) -> HttpResponse {
    let participates = new_part.into_inner();
    let token_data = req_user.unwrap();

    if token_data.role == "regular" && participates.uid != token_data.id {
        return HttpResponse::Unauthorized().finish();
    }

    if !participates.is_valid() {
        return HttpResponse::BadRequest().finish();
    }

    let val = web::block(move || {
        let mut conn = pool.get().unwrap();
        participates_repository::update_participation(&mut conn, participates)
    }).await.unwrap();

    match val {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

#[get("/api/participates")]
async fn all_participates(pool: Data<DbPool>, query: web::Query<ParticipationPaginationDTO>) -> HttpResponse {
    let participations = web::block(move || {
        let mut conn = pool.get().unwrap();
        let participates = participates_repository::get_participation_paginated(&mut conn, query.into_inner()).unwrap();

        let mut res = vec![];
        for part in participates {
            let user = users_repo::get_user_by_id(&mut conn, part.uid).unwrap().unwrap();
            let contest = contest_repository::get_contest_by_id(&mut conn, part.cid).unwrap().unwrap();
            res.push(ParticipatesDTO{participates: part, user, contest});
        }
        res
    }).await.map_err(|_| HttpResponse::InternalServerError().finish());

    match participations {
        Ok(mut v) => {
            v.sort_by(|a, b| (a.participates.uid, a.participates.cid).cmp(&(b.participates.uid, b.participates.cid)));
            HttpResponse::Ok().json(v)
        },
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

#[get("/api/participates/{id1}/{id2}")]
async fn get_participates_by_id(pool: Data<DbPool>, path: Path<(i32, i32)>) -> HttpResponse {
    let id = path.into_inner();

    let part = web::block(move || {
        let mut conn = pool.get().unwrap();
        participates_repository::get_participation_by_ids(&mut conn, id.0, id.1)
    }).await.unwrap().map_err(|_| HttpResponse::InternalServerError().finish());

    match part {
        Ok(v) => HttpResponse::Ok().json(v),
        Err(_) => HttpResponse::InternalServerError().finish()
    }

}