use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Post {
    pub title: String,
    pub body: String,
}
