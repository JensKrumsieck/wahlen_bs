use crate::{http::AppContext, Result};
use axum::{extract::State, routing::get, Json, Router};
use serde::Serialize;

#[derive(Debug, Serialize, utoipa::ToSchema, PartialEq)]
pub(crate) struct PartiesResponse<T> {
    parties: Vec<T>,
}

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
    (status = 200, description = "List all parties", body = PartiesResponse<Party>)
))]
async fn get_parties(ctx: State<AppContext>) -> Result<Json<PartiesResponse<Party>>> {
    let parties = sqlx::query_as!(Party, "SELECT * FROM party")
        .fetch_all(&ctx.db)
        .await?;
    Ok(Json(PartiesResponse { parties }))
}
