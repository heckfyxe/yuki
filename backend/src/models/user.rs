use crate::utils::serde::hash_with_argon;
use mongodb::bson::serde_helpers::serialize_hex_string_as_object_id;
use serde::{Deserialize, Serialize};

pub use crate::repositories::user::Repository;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(alias = "_id", serialize_with = "serialize_hex_string_as_object_id")]
    pub id: String,
    pub nickname: String,
    pub name: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Register {
    pub nickname: String,
    pub name: String,
    #[serde(serialize_with = "hash_with_argon")]
    pub password: String,
}
