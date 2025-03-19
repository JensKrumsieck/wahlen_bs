use crate::AppContext;
use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub(crate) struct Party {
    pub id: i64,
    pub name: String,
    pub abbreviation: String,
}

pub(crate) fn router() -> Router<AppContext> {
    Router::new().route("/party", get(get_parties))
}

async fn get_parties(ctx: State<AppContext>) -> Result<Json<Vec<Party>>, StatusCode> {
    let parties = sqlx::query_as!(Party, "SELECT * FROM party")
        .fetch_all(&ctx.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(parties))
}
