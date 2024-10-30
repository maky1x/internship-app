use axum::{response::IntoResponse, routing::get, Json, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/api/test-route", get(test_route_check));

    println!("Server started successfully at 0.0.0.0:8080");

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app.into_make_service())
    .await
    .unwrap();
}

pub async fn test_route_check() -> impl IntoResponse {
    const MESSAGE: &str = "API service";

    let json_response = serde_json::json!({
        "status": "ok",
        "message": MESSAGE
    });

    Json(json_response)
}