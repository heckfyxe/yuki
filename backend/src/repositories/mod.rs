use actix_web::web;
use actix_web::web::ServiceConfig;
use mongodb::Database;
pub mod user;

pub trait BaseRepository {
    fn collection_name() -> &'static str;
}

pub fn init(database: Database) -> impl FnOnce(&mut ServiceConfig) {
    move |cfg: &mut ServiceConfig| {
        cfg.app_data(web::Data::new(user::Repository::new(database)));
    }
}
