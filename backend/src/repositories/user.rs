use crate::models::error::ApiError;
use crate::models::user;
use crate::models::user::User;
use crate::repositories::BaseRepository;

use bson::{to_document, Bson, Document};
use log::debug;
use mongodb::bson::doc;
use mongodb::{Collection, Database};

use crate::utils::user::{check_password, hash_password};

pub struct Repository {
    collection: Collection<Document>,
}

impl BaseRepository for Repository {
    fn collection_name() -> &'static str {
        "users"
    }
}

impl Repository {
    pub fn new(database: Database) -> Self {
        Self {
            collection: database.collection(Self::collection_name()),
        }
    }

    pub async fn sign_up(&self, mut user: user::Register) -> Result<Option<Bson>, ApiError> {
        let nickname_busy = self
            .collection
            .find_one(doc! { "nickname": user.nickname.clone() }, None)
            .await?
            .is_some();

        if nickname_busy {
            return Result::Ok(None);
        }

        user.password = hash_password(user.password).await?;

        let record = self
            .collection
            .insert_one(to_document(&user).unwrap(), None)
            .await?;
        Ok(Some(record.inserted_id))
    }

    pub async fn login(&self, login: user::Login) -> Result<Option<User>, ApiError> {
        let user = self
            .collection
            .find_one(doc! { "nickname": login.nickname.clone() }, None)
            .await?;

        if user.is_none() {
            return Ok(None);
        }

        debug!("Login: {:?}", user);
        let user: User = bson::from_document(user.unwrap()).unwrap();

        let password_correct = check_password(user.password.clone(), login.password).await?;

        match password_correct {
            true => Ok(Some(user)),
            false => Ok(None),
        }
    }
}
