use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateBlogpostSchema {
    pub main: String,
    pub username: String
}