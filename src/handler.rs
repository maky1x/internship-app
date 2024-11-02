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
    }
}

pub async fn blogpost_list_handler(
    State(data): State<Arc<AppState>> 
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
 let blogposts = sqlx::query_as!(
    BlogpostModel,
    r#"SELECT * FROM blogposts"#
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
    let query_result = sqlx::query(r#"INSERT INTO blogposts (id, main, username) VALUES (?, ?, ?)"#)
    .bind(&id)
    .bind(&body.main)
    .bind(&body.username)
    .execute(&data.db)
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

pub async fn test_route_check() -> impl IntoResponse {
    const MESSAGE: &str = "API service";

    let json_response = serde_json::json!({
        "status": "ok",
        "message": MESSAGE
    });

    Json(json_response)
}