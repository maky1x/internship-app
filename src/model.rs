use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct BlogpostModel {
    pub id: String,
    pub main: String,
    pub username: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct BlogpostModelResponse {
    pub id: String,
    pub main: String,
    pub username: String,
    pub created_at: chrono::DateTime<chrono::Utc>
}