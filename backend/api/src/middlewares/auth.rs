use crate::models::error::ApiError;
use actix_web::dev::Payload;
use actix_web::http::header;
use actix_web::{FromRequest, HttpRequest};
use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::models::claims::Claims;
use jsonwebtoken::errors::ErrorKind;
use std::future::{ready, Ready};

impl FromRequest for Claims {
    type Error = ApiError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        ready(extract_auth(req))
    }
}

fn extract_token(req: &HttpRequest) -> Result<String, ApiError> {
    let authorization = req
        .headers()
        .get(header::AUTHORIZATION)
        .ok_or_else(|| ApiError::AuthError("Missing authorization header".to_string()))?
        .to_str()
        .map_err(|_| ApiError::AuthError("Malformed authorization header".to_string()))?;

    if authorization.len() > 7 && authorization[0..7].eq_ignore_ascii_case("bearer ") {
        Ok(authorization[7..].to_string())
    } else {
        Err(ApiError::AuthError(
            "Invalid authentication scheme".to_string(),
        ))
    }
}

fn extract_auth(req: &HttpRequest) -> Result<Claims, ApiError> {
    let token = extract_token(req)?;
    Ok(decode(
        token.as_str(),
        &DecodingKey::from_secret(dotenv::var("JWT_SECRET").unwrap().as_bytes()),
        &Validation::default(),
    )
    .map_err(|e| match e.kind() {
        ErrorKind::ExpiredSignature => ApiError::AuthError("Token expired".to_string()),
        _ => ApiError::AuthError("Invalid token".to_string()),
    })?
    .claims)
}
