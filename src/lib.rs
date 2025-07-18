use std::time::Duration;

use axum::{response::Redirect, routing::get, Router};
use models::{election, party, region};
use sqlx::SqlitePool;
use tokio::net::TcpListener;
use tower_http::{
    catch_panic::CatchPanicLayer,
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};
use tracing::info;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub mod models;

#[derive(Clone)]
pub(crate) struct AppContext {
    db: SqlitePool,
}

#[derive(OpenApi)]
#[openapi(paths(
    region::get_regions,
    party::get_parties,
    election::get_election,
    election::get_elections
))]
struct ApiDoc;

pub async fn serve(db: SqlitePool) -> anyhow::Result<()> {
    let ctx = AppContext { db };
    let app = router(ctx);
    let listener = TcpListener::bind("0.0.0.0:8080").await?;

    info!("Listening on http://{}", listener.local_addr()?);
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

fn router(ctx: AppContext) -> Router {
    let api = ApiDoc::openapi();

    Router::new()
        .route("/", get(|| async { Redirect::temporary("/docs") }))
        .merge(SwaggerUi::new("/docs").url("/docs/openapi.json", api))
        .merge(election::router())
        .merge(party::router())
        .merge(region::router())
        .layer((
            TraceLayer::new_for_http().on_failure(()),
            CompressionLayer::new(),
            CatchPanicLayer::new(),
            TimeoutLayer::new(Duration::from_secs(20)),
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([axum::http::Method::GET])
                .allow_headers(Any),
        ))
        .with_state(ctx)
}
