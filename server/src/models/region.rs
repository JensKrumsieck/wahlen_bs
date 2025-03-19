use super::vote::{ElectoralVote, VoteTurnout};
use crate::AppContext;
use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use serde::Serialize;

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub(crate) struct Region {
    #[schema(example = 120)]
    pub id: i64,
    #[schema(example = "Östliches Ringgebiet")]
    pub name: String,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub(crate) struct RegionVotes {
    #[serde(flatten)]
    pub region: Region,
    pub turnout: Vec<VoteTurnout>,
    pub votes: Vec<ElectoralVote>,
}

pub(crate) fn router() -> Router<AppContext> {
    Router::new().route("/region", get(get_regions))
}

#[utoipa::path(get, path = "/region",
responses(
    (status = 200, description = "List all regions", body = [Region])
))]
async fn get_regions(ctx: State<AppContext>) -> Result<Json<Vec<Region>>, StatusCode> {
    let elections = sqlx::query_as!(Region, "SELECT * FROM region")
        .fetch_all(&ctx.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(elections))
}
