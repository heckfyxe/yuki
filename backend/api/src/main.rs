use std::io;

use crate::routes::chat as chat_routes;
use crate::routes::user as user_routes;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};

mod middlewares;
mod models;
mod repositories;
mod routes;
mod utils;

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let client = mongodb::Client::with_uri_str(dotenv::var("MONGO_URI").unwrap())
        .await
        .unwrap();
    let database = client.database(&dotenv::var("MONGO_DATABASE").unwrap()[..]);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .configure(repositories::init(database.clone()))
            .configure(user_routes::init)
            .configure(chat_routes::init)
    })
    .bind(format!("127.0.0.1:{}", dotenv::var("API_PORT").unwrap()))?
    .run()
    .await
}
