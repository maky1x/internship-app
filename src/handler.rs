use std::sync::Arc;

use axum::{
    extract::{State},
    http::StatusCode,
    response::IntoResponse,
    Json
};

use serde_json::json;

use crate::{
    model::{BlogpostModel, BlogpostModelResponse},
    schema::{CreateBlogpostSchema},
    AppState
};

fn to_blogpost_response(blogpost: &BlogpostModel) -> BlogpostModelResponse {
    BlogpostModelResponse {
        id: blogpost.id.to_owned(),
        main: blogpost.main.to_owned(),
        username: blogpost.username.to_owned(),
        created_at: blogpost.created_at.unwrap(),
        image: blogpost.image.clone().unwrap_or_default(),
        avatar: blogpost.avatar.clone().unwrap_or_default(),
    }
}

pub async fn blogpost_list_handler(
    State(data): State<Arc<AppState>> 
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
 let blogposts = sqlx::query_as!(
    BlogpostModel,
    r#"SELECT * FROM blogposts ORDER BY created_at"#
 )
 .fetch_all(&data.db)
 .await
 .map_err(|e| {
    let error_response = serde_json::json!({
        "status": "error",
        "message": format!("Database error: { }", e)
    });
    (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
 })?;

 let blogpost_responses = blogposts
 .iter()
 .map(|blogpost| to_blogpost_response(&blogpost))
 .collect::<Vec<BlogpostModelResponse>>();

 let json_response = serde_json::json!({
    "status": "ok",
    "count": blogpost_responses.len(),
    "blogposts": blogpost_responses
 });

 Ok(Json(json_response))
}

pub async fn create_blogpost_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateBlogpostSchema>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id = uuid::Uuid::new_v4().to_string();
    let mut image_str1 = "";
    let mut image_str2 = "";
    let mut avatar_str1 = "";
    let mut avatar_str2 = "";

    if &body.image != "" {
        image_str1 = ", image";
        image_str2 = ", ?";
    }

    if &body.avatar != "" {
        avatar_str1 = ", avatar";
        avatar_str2 = ", ?";
    }
    let query_str = format!(r#"INSERT INTO blogposts (id, main, username{}{}) VALUES (?, ?, ?{}{})"#, image_str1, avatar_str1, image_str2, avatar_str2);

    let mut query = sqlx::query(&query_str)
    .bind(&id)
    .bind(&body.main)
    .bind(&body.username);

    if !&body.image.is_empty() {
        query = query.bind(&body.image);
    }

    if !&body.avatar.is_empty() {
        query = query.bind(&body.avatar);
    }
    let query_result = query.execute(&data.db)
    .await
    .map_err(|err: sqlx::Error| err.to_string());

    if let Err(err) = query_result {
        if err.contains("Duplicate entry") {
            let error_response = serde_json::json!({
                "status": "error",
                "message": "Blogpost already exists"
            });
            return Err((StatusCode::CONFLICT, Json(error_response)));
        }

        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "status": "error", 
                "message": format!("{:?}", err)
            }))
        ));
    }

    let blogpost = sqlx::query_as!(BlogpostModel, r#"SELECT * FROM blogposts WHERE id = ?"#, &id)
    .fetch_one(&data.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "status": "error", 
                "message": format!("{:?}", e)
            }))
        )
    })?;

    let blogpost_response = serde_json::json!({
        "status": "success",
        "data": serde_json::json!({
            "blogpost": to_blogpost_response(&blogpost)
        })
    });

    Ok(Json(blogpost_response))
}