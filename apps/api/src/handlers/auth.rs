
use axum::{
    extract::State,
    http::StatusCode,
    Json
};
use crate::{
    models::auth::{
        AuthResponse,
        RegisterRequest
    },
    utils::password::hash_password,
    AppState
};

pub async fn register_handler(
    State(state):State<AppState>,
    Json(payload):Json<RegisterRequst>
) -> Result<Json<AuthResponse>,StatusCode> {
    let password_hash = hash_password(&payload.password)
    .map_err(|_| StatusCode :: INTERNAL_SERVER_ERROR)?;

    let result = sqlx::query!(
        r#"
        INSERT INTO users(name,email,password_hash)
        VALUES($1,$2,$3)
        "#,
        payload.name,
        payload.email,
        password_hash,        
    )
    .execute(&state.db)
    .await;

    match result{
        Ok(_) => Ok(Json(AuthResponse {
            message: "User registered successfully".to_string(),
        }))
    }


}