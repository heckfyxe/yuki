use crate::models::user;
use crate::repositories::BaseRepository;
use bson::{to_document, Bson, Document};
use mongodb::bson::doc;
use mongodb::error::Result;
use mongodb::{Collection, Database};

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

    pub async fn sign_up(&self, user: &user::Register) -> Result<Option<Bson>> {
        let nickname_busy = self
            .collection
            .find_one(doc! { "nickname": user.nickname.clone() }, None)
            .await?
            .is_some();

        if nickname_busy {
            return Result::Ok(None);
        }

        let record = self
            .collection
            .insert_one(to_document(user).unwrap(), None)
            .await?;
        Ok(Some(record.inserted_id))
    }
}
