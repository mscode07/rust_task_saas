use std::{env, net::SocketAddr};
use anyhow::Result;
use dotenvy::dotenv; 
use serde:: Serialize;
use axum::{
    extract::State,
    routing::get,
    Json,Router,
};
use sqlx::{
    postgres::PgPoolOptions,
    PgPool,
};

#[derive(Clone)]
struct AppState{
    db:PgPool,
}

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    message: String,
}


#[tokio :: main]
async fn main()-> Result<()>{
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")?;

    let port:u16 = env::var("APP_PORT")
    .unwrap_or_else(|_| "3000".to_string())
    .parse()?;

    println!("Connecting to Database...");

    let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(&database_url)
    .await?;

    println!("Database connected successfully"); 

    let state = AppState{db:pool};

    let app = Router::new()
    .route("/health",get(health_handler))
    .with_state(state);

    let addr = SocketAddr::from(([127,0,0,1],port));

    println!("erver running at http://{}",addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn health_handler(
    State(_state): State<AppState>,
) -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        message: "Task Management API is running".to_string(),
    })
}