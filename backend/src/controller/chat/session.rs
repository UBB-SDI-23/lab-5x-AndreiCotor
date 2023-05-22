use std::time::{Duration, Instant};
use actix::prelude::*;
use actix_web::web;
use actix_web::web::Data;
use actix_web_actors::ws;
use serde::Serialize;
use crate::controller::chat::server;
use crate::DbPool;
use crate::model::chat::Chat;
use crate::repository::chat_repo;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct WsChatSession {
    pub id: String,
    pub hb: Instant,
    pub name: Option<String>,
    pub addr: Addr<server::ChatServer>,
    pub db_pool: Data<DbPool>
}

impl WsChatSession {
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                act.addr.do_send(server::Disconnect {
                    id: act.id.clone(),
                    author: act.name.clone().unwrap_or("".to_string())
                });
                ctx.stop();
                return;
            }

            ctx.ping(b"");
        });
    }
}

impl Actor for WsChatSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);

        let addr = ctx.address();
        self.addr
            .send(server::Connect {
                addr: addr.recipient(),
                author: self.name.clone().unwrap_or("".to_string()),
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = res,
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.addr.do_send(server::Disconnect {
            id: self.id.clone(),
            author: self.name.clone().unwrap_or("".to_string())
        });
        Running::Stop
    }
}

impl Handler<server::Message> for WsChatSession {
    type Result = ();

    fn handle(&mut self, msg: server::Message, ctx: &mut Self::Context) {
        ctx.text(serde_json::to_string(&msg).unwrap());
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsChatSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            }
            ws::Message::Text(text) => {
                let m = text.trim();
                let msg: server::Message = serde_json::from_str(m).unwrap();

                if self.name.is_none() {
                    self.name = Some(msg.author);
                }

                self.addr.do_send(server::ClientMessage {
                    id: self.id.clone(),
                    msg: msg.message.clone(),
                    author: self.name.clone().unwrap(),
                    uid: msg.uid
                });

                let mut conn = self.db_pool.get().unwrap();
                chat_repo::add_chat(&mut conn, Chat {
                    nickname: self.name.clone().unwrap(),
                    message: msg.message,
                    uid: msg.uid,
                })
            }
            ws::Message::Binary(_) => println!("Unexpected binary"),
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            ws::Message::Continuation(_) => {
                ctx.stop();
            }
            ws::Message::Nop => (),
        }
    }
}