use bson::serde_helpers::deserialize_hex_string_from_object_id;
use serde::{Deserialize, Serialize};

pub use crate::repositories::user::Repository;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(
        alias = "_id",
        deserialize_with = "deserialize_hex_string_from_object_id"
    )]
    pub id: String,
    pub nickname: String,
    pub name: String,
    #[serde(skip_serializing)]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Register {
    pub nickname: String,
    pub name: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Login {
    pub nickname: String,
    pub password: String,
}
