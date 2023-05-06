use actix_cors::Cors;
use std::env;
use actix_web::{App, HttpServer};
use actix_web::web::Data;
use actix_web_httpauth::middleware::HttpAuthentication;
use diesel::{PgConnection};
use dotenvy::dotenv;
use crate::controller::problem_controller::problem_config;
use crate::controller::user_controller::user_config;
use crate::controller::submission_controller::submission_config;
use diesel::r2d2::ConnectionManager;
use crate::controller::authentication_controller::authentication_config;
use crate::controller::contest_controller::contest_config;
use crate::controller::participates_controller::participates_config;
use crate::middleware::authentication_validator;
use crate::utils::mock::MockablePool;

// r2d2 Pool is already included in the definition
type DbPool = MockablePool<ConnectionManager<PgConnection>>;

mod controller;
mod model;
mod schema;
mod repository;
mod utils;
mod middleware;

fn connect_to_db() -> ConnectionManager<PgConnection> {
    dotenv().expect(".env file not found");

    let database_url = env::var("DATABASE_URL").expect("Database URL must be set");
    ConnectionManager::<PgConnection>::new(database_url)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = r2d2::Pool::builder()
        .build(connect_to_db())
        .expect("Failed to create db connection pool!");

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(Data::new(MockablePool::Real(pool.clone())))
            .configure(authentication_config)
            .configure(problem_config)
            .configure(user_config)
            .configure(submission_config)
            .configure(contest_config)
            .configure(participates_config)
    }).bind(("127.0.0.1", 8000))?
        .run().await
}
