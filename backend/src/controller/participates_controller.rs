use actix_web::{HttpResponse, web, post, delete, put, get};
use actix_web::web::{Data, Json, Path};
use crate::DbPool;
use crate::model::participates::Participates;
use crate::repository::participates_repository;

pub fn participates_config(cfg: &mut web::ServiceConfig) {
    cfg.service(add_participates)
        .service(delete_participates)
        .service(update_participates)
        .service(all_participates)
        .service(get_participates_by_id);
}

#[post("/api/participates")]
async fn add_participates(pool: Data<DbPool>, participates_json: Json<Vec<Participates>>) -> HttpResponse {
    let participates_list = participates_json.into_inner();
    for participates in &participates_list {
        if !participates.is_valid() {
            return HttpResponse::BadRequest().finish();
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
async fn delete_participates(pool: Data<DbPool>, path: Path<(i32, i32)>) -> HttpResponse {
    let id = path.into_inner();
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
async fn update_participates(pool: Data<DbPool>, new_part: Json<Participates>) -> HttpResponse {
    let participates = new_part.into_inner();
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
async fn all_participates(pool: Data<DbPool>) -> HttpResponse {
    let participations = web::block(move || {
        let mut conn = pool.get().unwrap();
        participates_repository::get_all_participation(&mut conn)
    }).await.unwrap().map_err(|_| HttpResponse::InternalServerError().finish());

    match participations {
        Ok(v) => HttpResponse::Ok().json(v),
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