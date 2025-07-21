use crate::http::AppContext;
use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use serde::Serialize;

#[derive(Debug, Serialize, utoipa::ToSchema, PartialEq)]
pub(crate) struct Party {
    #[schema(example = "GRÜNE")]
    pub id: String,
    #[schema(example = "BÜNDNIS 90/DIE GRÜNEN")]
    pub name: String,
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
