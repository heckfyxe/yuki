use argon2::Config;
use rand::Rng;
use tokio::task::JoinError;

pub async fn hash_password(password: String) -> Result<String, JoinError> {
    actix_web::rt::task::spawn_blocking(move || {
        let salt: [u8; 32] = rand::thread_rng().gen();
        argon2::hash_encoded(password.as_bytes(), &salt, &Config::default()).unwrap()
    })
    .await
}

pub async fn check_password(encoded: String, password: String) -> Result<bool, JoinError> {
    actix_web::rt::task::spawn_blocking(move || {
        argon2::verify_encoded(encoded.as_str(), password.as_bytes()).unwrap()
    })
    .await
}
