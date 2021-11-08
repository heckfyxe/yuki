use crate::models::claims::Claims;
use actix_web::get;
use actix_web::web;
use actix_web::Responder;

#[get("/content")]
async fn chats(claims: Claims) -> impl Responder {
    format!("User id: {}", claims.sub)
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(chats);
}
