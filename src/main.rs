use axum::{http::StatusCode, routing::get, Extension, Json, Router};
use sqlx::SqlitePool;
use tokio::net::TcpListener;
use wahlen_bs::models::Election;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let database_url = dotenvy::var("DATABASE_URL")?;
    let pool = SqlitePool::connect(&database_url).await?;

    let app = Router::new().route("/election", get(get_elections).layer(Extension(pool)));
    let listener = TcpListener::bind("0.0.0.0:8080").await?;

    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn get_elections(db: Extension<SqlitePool>) -> Result<Json<Vec<Election>>, StatusCode> {
    let elections = sqlx::query_as!(Election, "SELECT * FROM election")
        .fetch_all(&*db)
        .await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(elections))
}
