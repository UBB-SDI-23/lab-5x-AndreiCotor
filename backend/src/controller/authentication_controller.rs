use actix_web::{HttpResponse, web, post, get};
use actix_web::web::{Data, Json, Path};
use password_hash::{PasswordHash, Salt};
use crate::DbPool;
use crate::model::user_credentials::{InsertableUserCredentials, NewUserCredentials};
use crate::repository::{user_credentials_repo, users_repo};
use argon2::Argon2;
use chrono::{Duration, Utc};
use crate::model::dto::token_claims::TokenClaims;
use jwt::SignWithKey;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use uuid::Uuid;
use crate::model::dto::login_dto::LoginDTO;
use crate::model::user::User;

pub fn authentication_config(cfg: &mut web::ServiceConfig) {
    cfg.service(login)
        .service(register)
        .service(confirm);
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

    if !user.confirmed {
        return HttpResponse::BadRequest().json("Account is not confirmed!")
    }

    let password_hash = PasswordHash::new(&user.password).unwrap();
    match password_hash.verify_password(&[&Argon2::default()], credential.password) {
        Ok(_) => {
            let jwt_secret: Hmac<Sha256> = Hmac::new_from_slice(
                std::env::var("JWT_SECRET")
                    .expect("JWT_SECRET must be set!")
                    .as_bytes(),
            ).unwrap();

            let claims = TokenClaims { id: user.id, role: user.role.clone() };
            let token_str = claims.sign_with_key(&jwt_secret).unwrap();
            HttpResponse::Ok().json(LoginDTO{id: user.id, token: token_str, username: user.username, role: user.role })
        }
        Err(_) => {
            HttpResponse::BadRequest().json("Invalid password!")
        }
    }
}

#[post("/api/register")]
async fn register(pool: Data<DbPool>, credential_json: Json<NewUserCredentials>) -> HttpResponse {
    let mut credential = credential_json.into_inner();

    if credential.password.len() < 8 {
        return HttpResponse::BadRequest().json("Password must be longer than 8 characters");
    }

    credential.password = PasswordHash::generate(Argon2::default(), credential.password, Salt::from_b64("YW5kcmVpZWJvc3Nz").unwrap()).unwrap().to_string();
    let uuid = Uuid::new_v4().to_string();

    let ins_credential = InsertableUserCredentials {
        username: credential.username,
        password: credential.password,
        uuid: uuid.clone()
    };

    let res = web::block(move || {
        let mut conn = pool.get().unwrap();
        let res = user_credentials_repo::add_user_credentials(&mut conn, ins_credential.clone());
        if res.is_err() {
            return Err(());
        }
        let added = user_credentials_repo::get_user_credentials(&mut conn, ins_credential.username).unwrap();
        users_repo::add_user(&mut conn, User::from_id(added.id, added.username));

        Ok(())
    }).await.unwrap();

    match res {
        Ok(_) => HttpResponse::Ok().json(uuid),
        Err(_) => HttpResponse::BadRequest().json("Username already in use")
    }
}

#[get("/api/register/confirm/{uuid}")]
async fn confirm(pool: Data<DbPool>, uuid: Path<String>) -> HttpResponse {
    match web::block(move || {
        let mut conn = pool.get().unwrap();
        let mut credentials = user_credentials_repo::get_user_credentials_by_uuid(&mut conn, uuid.into_inner()).unwrap();

        if credentials.created + Duration::minutes(10) >= Utc::now().naive_utc() {
            credentials.confirmed = true;
            user_credentials_repo::update_user_credentials(&mut conn, credentials).expect("");
            Ok(())
        }
        else {
            Err(())
        }
    }).await.unwrap() {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::BadRequest().json("We were unable to confirm your account")
    }
}

