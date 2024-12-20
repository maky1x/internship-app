use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router
};

use crate::{
    handler::{
        create_blogpost_handler, blogpost_list_handler
    }, 
    AppState
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
    .route("/api/blogposts", post(create_blogpost_handler))
    .route("/api/blogposts", get(blogpost_list_handler))
    .with_state(app_state)
}