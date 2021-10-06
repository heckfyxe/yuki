use argon2::Config;

use rand::Rng;
use serde::{ser, Serializer};

pub fn hash_with_argon<S: Serializer>(val: &str, serializer: S) -> Result<S::Ok, S::Error> {
    let salt: [u8; 32] = rand::thread_rng().gen();
    match argon2::hash_encoded(val.as_bytes(), &salt, &Config::default()) {
        Ok(hash) => serializer.serialize_str(&hash[..]),
        Err(_) => Err(ser::Error::custom(format!(
            "cannot hash {} with argon",
            val
        ))),
    }
}
