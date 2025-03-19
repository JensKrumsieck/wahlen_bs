use crate::{models::region::Region, AppContext};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};

use super::{
    region::RegionVotes,
    vote::{fetch_votes, Turnout},
};

#[derive(Debug, Serialize)]
struct Election {
    pub id: i64,
    pub date: i64,
    pub name: String,
}

#[derive(Debug, Serialize)]
struct ElectionRegion {
    #[serde(flatten)]
    pub election: Election,
    pub turnout: Vec<Turnout>,
    pub region: RegionVotes,
}

pub(crate) fn router() -> Router<AppContext> {
    Router::new().route("/election", get(get_elections)).route(
        "/election/{election_id}/region/{region_id}",
        get(get_election_region),
    )
}

async fn get_elections(ctx: State<AppContext>) -> Result<Json<Vec<Election>>, StatusCode> {
    let elections = sqlx::query_as!(Election, "SELECT * FROM election")
        .fetch_all(&ctx.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(elections))
}

#[derive(Deserialize)]
struct ElectionRegionQuery {
    primary_vote: Option<bool>,
}

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

    let mut turnout = sqlx::query_as!(
        Turnout,
        "SELECT * FROM turnout WHERE election_id = $1 AND region_id = $2",
        election_id,
        region_id
    )
    .fetch_all(&ctx.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

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
