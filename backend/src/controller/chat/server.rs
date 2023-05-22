use std::collections::{HashMap, HashSet};
use uuid::Uuid;
use actix::prelude::*;
use diesel::serialize::IsNull::No;
use serde::{Deserialize, Serialize};

#[derive(Message)]
#[derive(Serialize, Deserialize, Debug)]
#[rtype(result = "()")]
pub struct Message{
    pub message: String,
    pub author: String,
    pub uid: Option<i32>
}

#[derive(Message)]
#[rtype(String)]
pub struct Connect {
    pub addr: Recipient<Message>,
    pub author: String
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: String,
    pub author: String
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage {
    pub id: String,
    pub msg: String,
    pub author: String,
    pub uid: Option<i32>
}

#[derive(Debug)]
pub struct ChatServer {
    sessions: HashMap<String, Recipient<Message>>,
    connected: HashSet<String>
}

impl ChatServer {
    pub fn new() -> ChatServer {
        ChatServer {
            sessions: HashMap::new(),
            connected: HashSet::new()
        }
    }

    fn send_message(&self, message: &str, skip_id: String, author: &str, uid: Option<i32>) {
        for id in &self.connected {
            if *id != skip_id {
                if let Some(addr) = self.sessions.get(id) {
                    addr.do_send(Message {
                        message: message.to_owned(),
                        author: author.to_owned(),
                        uid
                    });
                }
            }
        }
    }
}

impl Actor for ChatServer {
    type Context = Context<Self>;
}

impl Handler<Connect> for ChatServer {
    type Result = String;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        self.send_message((msg.author + " joined").as_str(),
                          Uuid::new_v4().to_string(),
                          "Server",
                          None
        );

        let id = Uuid::new_v4().to_string();
        self.sessions.insert(id.clone(), msg.addr);
        self.connected.insert(id.clone());

        id
    }
}

impl Handler<Disconnect> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        self.sessions.remove(msg.id.as_str());
        self.connected.remove(msg.id.as_str());
        self.send_message((msg.author + " disconnected").as_str(),
                          Uuid::new_v4().to_string(),
                          "Server",
                          None
        );
    }
}

impl Handler<ClientMessage> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Context<Self>) {
        self.send_message(msg.msg.as_str(), msg.id, msg.author.as_str(), msg.uid);
    }
}
