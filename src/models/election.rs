use super::{region::RegionVotes, vote::fetch_votes};
use crate::{
    models::{region::Region, vote::VoteTurnout},
    AppContext,
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, utoipa::ToSchema)]
struct Election {
    #[schema(example = 1)]
    pub id: i64,
    #[schema(example = 2025)]
    pub date: i64,
    #[schema(example = "Bundestagswahl")]
    pub name: String,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
struct ElectionRegion {
    #[serde(flatten)]
    pub election: Election,
    pub turnout: Vec<VoteTurnout>,
    pub region: RegionVotes,
}

pub(crate) fn router() -> Router<AppContext> {
    Router::new().route("/election", get(get_elections)).route(
        "/election/{election_id}/region/{region_id}",
        get(get_election_region),
    )
}

#[utoipa::path(get, path = "/election",
responses(
    (status = 200, description = "List all elections", body = [Election])
))]
async fn get_elections(ctx: State<AppContext>) -> Result<Json<Vec<Election>>, StatusCode> {
    let elections = sqlx::query_as!(Election, "SELECT * FROM election")
        .fetch_all(&ctx.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(elections))
}

#[derive(Deserialize, utoipa::IntoParams)]
#[into_params(parameter_in=Query)]
struct ElectionRegionQuery {
    primary_vote: Option<bool>,
}

#[utoipa::path(get, path = "/election/{election_id}/region/{region_id}",
params(
    ("election_id" = i32, Path, description = "election database id"),
    ("region_id" = i32, Path, description = "region id"),
    ElectionRegionQuery
),
responses(
    (status = 200, description = "List Election results for specific Region", body = ElectionRegion)
))]
async fn get_election_region(
    Path((election_id, region_id)): Path<(i64, i64)>,
    Query(query): Query<ElectionRegionQuery>,
    ctx: State<AppContext>,
) -> Result<Json<ElectionRegion>, StatusCode> {
    let election = sqlx::query_as!(
        Election,
        "SELECT * FROM election WHERE id = $1",
        election_id
    )
    .fetch_one(&ctx.db)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    let region = sqlx::query_as!(Region, "SELECT * FROM region where id = $1", region_id)
        .fetch_one(&ctx.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let mut votes = fetch_votes(election_id, region_id, &ctx)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(primary) = query.primary_vote {
        votes.retain(|v| v.primary_vote == primary)
    }

    let region_votes = RegionVotes { region, votes };

    let raw_turnouts = sqlx::query!(
        "SELECT * FROM turnout WHERE election_id = $1 AND region_id = $2",
        election_id,
        region_id
    )
    .fetch_all(&ctx.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut turnout: Vec<VoteTurnout> = raw_turnouts
        .iter()
        .map(|t| VoteTurnout {
            eligible: t.eligible,
            voted: t.voted,
            primary_vote: t.primary_vote,
        })
        .collect();

    if let Some(primary) = query.primary_vote {
        turnout.retain(|v| v.primary_vote == primary)
    }

    let result = ElectionRegion {
        region: region_votes,
        election,
        turnout,
    };

    Ok(Json(result))
}
