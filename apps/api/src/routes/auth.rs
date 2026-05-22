use axum:: {
    routing:: post,
    Router,
};

use crate::{
    handler:: auth::register_handler,
    AppState,
};

pub fn auth_routes() -> Router<AppState>{
    Router::new()
    .route("/register",post(register_handler))
}