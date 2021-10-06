use crate::models::user::Register;
use crate::repositories::user;
use actix_web::{post, web, HttpResponse};
use mongodb::bson::bson;

#[post("/sign_up")]
async fn sign_up(
    user: web::Json<Register>,
    repository: web::Data<user::Repository>,
) -> HttpResponse {
    let result = repository.get_ref().sign_up(&user.into_inner()).await;
    match result {
        Ok(response) => match response {
            Some(id) => {
                HttpResponse::Created().json(bson!({ "id": id.as_object_id().unwrap().to_hex() }))
            }
            None => HttpResponse::Conflict().finish(),
        },
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(sign_up);
}
