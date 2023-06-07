use actix_web::dev::ServiceRequest;
use actix_web::{Error, HttpMessage};
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::extractors::{bearer::BearerAuth, bearer};
use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use sha2::Sha256;
use crate::model::dto::token_claims::TokenClaims;

pub async fn authentication_validator(req: ServiceRequest, credentials: BearerAuth)
                                      -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let config = req
        .app_data::<bearer::Config>()
        .cloned()
        .unwrap_or_default()
        .scope("");

    let token = credentials.token().to_owned();
    let jwt_secret: Hmac<Sha256> = Hmac::new_from_slice(
        std::env::var("JWT_SECRET")
            .expect("JWT_SECRET must be set!")
            .as_bytes(),
    ).unwrap();

    let claims: Result<TokenClaims, jwt::error::Error> = token.verify_with_key(&jwt_secret);
    match claims {
        Ok(value) => {
            req.extensions_mut().insert(value);
            Ok(req)
        },
        Err(_) => Err((AuthenticationError::from(config).into(), req))
    }
}

pub async fn admin_authentication_validator(req: ServiceRequest, credentials: BearerAuth)
    -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let config = req
        .app_data::<bearer::Config>()
        .cloned()
        .unwrap_or_default()
        .scope("");

    let token = credentials.token().to_owned();
    let jwt_secret: Hmac<Sha256> = Hmac::new_from_slice(
        std::env::var("JWT_SECRET")
            .expect("JWT_SECRET must be set!")
            .as_bytes(),
    ).unwrap();

    let claims: Result<TokenClaims, jwt::error::Error> = token.verify_with_key(&jwt_secret);
    match claims {
        Ok(value) => {
            if value.role != "admin" {
                return Err((AuthenticationError::from(config).into(), req));
            }
            
            req.extensions_mut().insert(value);
            Ok(req)
        },
        Err(_) => Err((AuthenticationError::from(config).into(), req))
    }
}