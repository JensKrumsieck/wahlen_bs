use super::vote::ElectoralVote;
use crate::AppContext;
use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub(crate) struct Region {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub(crate) struct RegionVotes {
    #[serde(flatten)]
    pub region: Region,
    pub votes: Vec<ElectoralVote>,
}

pub(crate) fn router() -> Router<AppContext> {
    Router::new().route("/region", get(get_regions))
}

async fn get_regions(ctx: State<AppContext>) -> Result<Json<Vec<Region>>, StatusCode> {
    let elections = sqlx::query_as!(Region, "SELECT * FROM region")
        .fetch_all(&ctx.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(elections))
}
