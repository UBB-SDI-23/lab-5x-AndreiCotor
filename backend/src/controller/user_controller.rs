use actix_web::{HttpResponse, web, get, delete, put, post};
use actix_web::web::{Data, Json, Path};
use actix_web_httpauth::middleware::HttpAuthentication;
use serde::Deserialize;
use crate::DbPool;
use crate::middleware::authentication_validator;
use crate::model::dto::pagination_dto::{PaginationDTO, StatisticPagination};
use crate::model::dto::user_dto::{UserDTO, UserReportDTO, UserSubmissionsDTO};
use crate::model::user::{NewUser, User};
use crate::repository::{submission_repository, users_repo};

pub fn user_config(cfg: &mut web::ServiceConfig) {
    cfg.service(all_users)
        .service(get_users_autocomplete)
        .service(get_user_by_id)
        .service(get_users_by_number_of_participations)
        .service(
            web::scope("")
                .wrap(HttpAuthentication::bearer(authentication_validator))
                .service(add_user)
                .service(delete_user)
                .service(update_user));
}

#[derive(Deserialize)]
struct Autocomplete {
    lname: Option<String>
}

#[post("/api/user")]
async fn add_user(pool: Data<DbPool>, new_user: Json<NewUser>) -> HttpResponse {
    web::block(move || {
        let mut conn = pool.get().unwrap();
        users_repo::add_user(&mut conn, new_user.into_inner());
    }).await.unwrap();
    HttpResponse::Ok().finish()
}

#[delete("/api/user/{id}")]
async fn delete_user(pool: Data<DbPool>, path: Path<i32>) -> HttpResponse {
    web::block(move || {
        let mut conn = pool.get().unwrap();
        users_repo::delete_user(&mut conn, path.into_inner());
    }).await.unwrap();

    HttpResponse::Ok().finish()
}

#[put("/api/user")]
async fn update_user(pool: Data<DbPool>, new_problem: Json<User>) -> HttpResponse {
    web::block(move || {
        let mut conn = pool.get().unwrap();
        users_repo::update_user(&mut conn, new_problem.into_inner());
    }).await.unwrap();

    HttpResponse::Ok().finish()
}

#[get("/api/user")]
async fn all_users(pool: Data<DbPool>, query: web::Query<PaginationDTO>) -> HttpResponse {
    let mut users = web::block(move || {
        let mut conn = pool.get().unwrap();
        let users = users_repo::get_users_paginated(&mut conn, query.into_inner()).unwrap();

        let mut res = vec![];
        for user in users {
            let cnt = submission_repository::get_all_submissions_by_user_id(&mut conn,user.id).unwrap().len() as i32;
            res.push(UserSubmissionsDTO{user, cnt});
        }
        res
    }).await.map_err(|_| HttpResponse::InternalServerError().finish()).unwrap();

    users.sort_by(|a, b| a.user.id.cmp(&b.user.id));

    HttpResponse::Ok().json(users)
}

#[get("/api/user/autocomplete")]
async fn get_users_autocomplete(pool: Data<DbPool>, path: web::Query<Autocomplete>) -> HttpResponse {
    let mut users = web::block(move || {
        let mut conn = pool.get().unwrap();
        users_repo::get_users_by_last_name(&mut conn, path.into_inner().lname)
    }).await.unwrap().map_err(|_| HttpResponse::InternalServerError().finish()).unwrap();

    HttpResponse::Ok().json(users)
}

#[get("/api/user/{id}")]
async fn get_user_by_id(pool: Data<DbPool>, path: Path<i32>) -> HttpResponse {
    let id = path.into_inner();

    let user = web::block(move || {
        let mut conn = pool.get().unwrap();
        let user = users_repo::get_user_by_id(&mut conn, id).unwrap().unwrap();
        let submission_list = submission_repository::get_all_submissions_by_user_id(&mut conn, id).unwrap();
        UserDTO {
            user,
            submissions: submission_list
        }
    }).await.unwrap();

    HttpResponse::Ok().json(user)
}

#[get("/api/user-by-number-of-participations")]
async fn get_users_by_number_of_participations(pool: Data<DbPool>, query: web::Query<StatisticPagination>) -> HttpResponse {
    let mut users = web::block(move || {
        let mut conn = pool.get().unwrap();
        users_repo::get_user_with_num_participations(&mut conn, query.into_inner()).unwrap().iter()
            .map(|(user, participations)| UserReportDTO{user: user.clone(), participations: *participations})
            .collect::<Vec<UserReportDTO>>()
    }).await.unwrap();

    users.sort_by(|a, b| (a.participations, a.user.id).cmp(&(b.participations, b.user.id)));

    HttpResponse::Ok().json(users)
}