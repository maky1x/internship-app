use axum::{response::IntoResponse, routing::get, Json, Router};
use tokio::net::TcpListener;
use std::sync::Arc;
use dotenv::dotenv;
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};

pub struct AppState {
    db: MySqlPool,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let pool = match MySqlPoolOptions::new()
    .max_connections(10)
    .connect(&database_url)
    .await
    {
        Ok(pool)=> {
            println!("Connection to database is successful");
            pool
        }
        Err(err)=> {
            println!("Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    let app = Router::new().route("/api/test-route", get(test_route_check))
    .with_state(Arc::new(AppState { db:pool.clone()}));

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