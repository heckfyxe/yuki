use crate::models::user::{Login, Register};
use crate::repositories::user;
use actix_web::{post, web, HttpResponse, Result};
use mongodb::bson::bson;

#[post("/sign_up")]
async fn sign_up(
    user: web::Json<Register>,
    repository: web::Data<user::Repository>,
) -> Result<HttpResponse> {
    let result = repository.get_ref().sign_up(user.into_inner()).await?;
    Ok(match result {
        Some(id) => {
            HttpResponse::Created().json(bson!({ "id": id.as_object_id().unwrap().to_hex() }))
        }
        None => HttpResponse::Conflict().finish(),
    })
}

#[post("/login")]
async fn login(
    user: web::Json<Login>,
    repository: web::Data<user::Repository>,
) -> Result<HttpResponse> {
    let result = repository.get_ref().login(user.into_inner()).await?;
    Ok(match result {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().finish(),
    })
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(sign_up);
    cfg.service(login);
}
