use axum::Router;
use models::{election, party, region};
use sqlx::SqlitePool;
use tokio::net::TcpListener;
use tracing::info;

pub mod models;

#[derive(Clone)]
pub(crate) struct AppContext {
    db: SqlitePool,
}

pub async fn serve(db: SqlitePool) -> anyhow::Result<()> {
    let ctx = AppContext { db };
    let app = router(ctx);
    let listener = TcpListener::bind("0.0.0.0:8080").await?;

    info!("Listening on http://{}", listener.local_addr()?);
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

fn router(ctx: AppContext) -> Router {
    Router::new()
        .merge(election::router())
        .merge(party::router())
        .merge(region::router())
        .with_state(ctx)
}
