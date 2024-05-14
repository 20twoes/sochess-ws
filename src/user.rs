use nanoid::nanoid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub name: String,
}

impl User {
    pub fn new() -> Self {
        Self {
            // TODO: Enforce uniqueness
            name: format!("anon{}", nanoid!(7)),
        }
    }
}
