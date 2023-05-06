use actix_web::{HttpResponse, web, post};
use actix_web::web::{Data, Json, Path};
use password_hash::{PasswordHash, PasswordHasher, Salt};
use crate::DbPool;
use crate::model::user_credentials::{NewUserCredentials};
use crate::repository::user_credentials_repo;
use argon2::Argon2;
use crate::model::dto::token_claims::TokenClaims;
use jwt::SignWithKey;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use crate::model::dto::login_dto::LoginDTO;

pub fn authentication_config(cfg: &mut web::ServiceConfig) {
    cfg.service(login)
        .service(register);
}

#[post("/api/login")]
async fn login(pool: Data<DbPool>, credential_json: Json<NewUserCredentials>) -> HttpResponse {
    let credential = credential_json.into_inner();
    let uc = web::block(move || {
        let mut conn = pool.get().unwrap();
        user_credentials_repo::get_user_credentials(&mut conn, credential.username)
    }).await.unwrap();

    let user = match uc {
        Some(inner) => inner,
        None => return HttpResponse::BadRequest().json("Invalid username!")
    };

    let password_hash = PasswordHash::new(&user.password).unwrap();
    match password_hash.verify_password(&[&Argon2::default()], credential.password) {
        Ok(_) => {
            let jwt_secret: Hmac<Sha256> = Hmac::new_from_slice(
                std::env::var("JWT_SECRET")
                    .expect("JWT_SECRET must be set!")
                    .as_bytes(),
            ).unwrap();

            let claims = TokenClaims { id: user.id };
            let token_str = claims.sign_with_key(&jwt_secret).unwrap();
            HttpResponse::Ok().json(LoginDTO{id: user.id, token: token_str, username: user.username })
        }
        Err(_) => {
            HttpResponse::BadRequest().json("Invalid password!")
        }
    }
}

#[post("/api/register")]
async fn register(pool: Data<DbPool>, credential_json: Json<NewUserCredentials>) -> HttpResponse {
    let mut credential = credential_json.into_inner();
    credential.password = PasswordHash::generate(Argon2::default(), credential.password, Salt::from_b64("YW5kcmVpZWJvc3Nz").unwrap()).unwrap().to_string();
    let res = web::block(move || {
        let mut conn = pool.get().unwrap();
        user_credentials_repo::add_user_credentials(&mut conn, credential)
    }).await.unwrap();

    match res {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::BadRequest().json("Username already in use")
    }
}

