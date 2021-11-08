use chrono::Duration;
use serde::{Deserialize, Serialize};
use std::ops::Add;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: i64,
    pub sub: String,
}

impl Claims {
    pub fn new(user_id: String) -> Self {
        Claims {
            sub: user_id,
            exp: chrono::Utc::now().add(Duration::days(1)).timestamp(),
        }
    }
}
