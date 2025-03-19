use crate::AppContext;
use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use serde::Serialize;

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub(crate) struct Party {
    #[schema(example = 3)]
    pub id: i64,
    #[schema(example = "BÜNDNIS 90/DIE GRÜNEN")]
    pub name: String,
    #[schema(example = "GRÜNE")]
    pub abbreviation: String,
}

pub(crate) fn router() -> Router<AppContext> {
    Router::new().route("/party", get(get_parties))
}

#[utoipa::path(get, path = "/party",
responses(
    (status = 200, description = "List all parties", body = [Party])
))]
async fn get_parties(ctx: State<AppContext>) -> Result<Json<Vec<Party>>, StatusCode> {
    let parties = sqlx::query_as!(Party, "SELECT * FROM party")
        .fetch_all(&ctx.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(parties))
}
