use std::time::Instant;
use actix_web::{HttpRequest, HttpResponse, web, get, Error};
use actix_web_actors::ws;
use actix::Addr;
use actix_web::web::Data;
use crate::controller::chat::{server, session};
use crate::DbPool;


pub fn chat_config(cfg: &mut web::ServiceConfig) {
    cfg.service(chat_route);
}

#[get("/api/chat/ws")]
async fn chat_route(req: HttpRequest, stream: web::Payload, pool: Data<DbPool>, srv: web::Data<Addr<server::ChatServer>>)
    -> Result<HttpResponse, Error> {
    ws::start(
        session::WsChatSession {
            id: "".to_string(),
            hb: Instant::now(),
            name: None,
            addr: srv.get_ref().clone(),
            db_pool: pool
        },
        &req,
        stream,
    )
}