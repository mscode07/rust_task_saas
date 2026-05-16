use std::env;
use anyhow::Result;
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;

#[tokio :: main]
async fn main()-> Result<()>{
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")?;

    let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(&database_url)
    .await?;

     sqlx::query("SELECT 1").execute(&pool).await?;

    println!("Database connectec successfulluy!");

    Ok(())
}