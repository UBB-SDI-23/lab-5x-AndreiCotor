use actix_web::{HttpResponse, web, get, delete, put, post};
use actix_web::web::{Data, Json, Path};
use crate::DbPool;
use crate::model::dto::user_dto::{UserDTO, UserReportDTO};
use crate::model::user::User;
use crate::repository::{submission_repository, users_repo};

pub fn user_config(cfg: &mut web::ServiceConfig) {
    cfg.service(add_user)
        .service(delete_user)
        .service(update_user)
        .service(all_users)
        .service(get_user_by_id)
        .service(get_users_by_number_of_participations);
}

#[post("/api/user")]
async fn add_user(pool: Data<DbPool>, new_user: Json<User>) -> HttpResponse {
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
async fn all_users(pool: Data<DbPool>) -> HttpResponse {
    let users = web::block(move || {
        let mut conn = pool.get().unwrap();
        users_repo::get_all_users(&mut conn)
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
async fn get_users_by_number_of_participations(pool: Data<DbPool>) -> HttpResponse {
    let mut users = web::block(move || {
        let mut conn = pool.get().unwrap();
        users_repo::get_all_users_with_participations(&mut conn).unwrap().iter()
            .map(|(user, participations)| UserReportDTO{user: user.clone(), participations: participations.len()})
            .collect::<Vec<UserReportDTO>>()
    }).await.unwrap();

    users.sort_by(|a, b| b.participations.cmp(&a.participations));

    HttpResponse::Ok().json(users)
}