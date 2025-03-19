use super::{party::Party, region::RegionVotes, vote::ElectoralVote};
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
use std::collections::HashMap;

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
    pub region: Vec<RegionVotes>,
}

pub(crate) fn router() -> Router<AppContext> {
    Router::new()
        .route("/election", get(get_elections))
        .route("/election/{election_id}", get(get_election))
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
struct ElectionQuery {
    primary_vote: Option<bool>,
    region: Option<i64>,
    party: Option<String>,
}

#[utoipa::path(get, path = "/election/{election_id}",
params(
    ("election_id" = i32, Path, description = "election database id"),
    ElectionQuery
),
responses(
    (status = 200, description = "List Election results", body = ElectionRegion)
))]
async fn get_election(
    Path(election_id): Path<i64>,
    Query(query): Query<ElectionQuery>,
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

    let result = sqlx::query!(
        r#"
        SELECT r.id as region_id, r.name as region_name,
               v.id as vote_id, v.votes, v.primary_vote,
               p.id as party_id, p.name as party_name,
               t.eligible, t.voted, t.primary_vote as turnout_primary_vote
        FROM region r
        LEFT JOIN vote v ON v.region_id = r.id AND v.election_id = $1
        LEFT JOIN party p ON v.party_id = p.id
        LEFT JOIN turnout t ON t.region_id = r.id AND t.election_id = $1
        WHERE ($2 IS NULL OR r.id = $2)
        AND ($3 IS NULL OR p.id = $3)
        AND ($4 IS NULL OR t.primary_vote = $4)
        AND ($4 IS NULL OR v.primary_vote = $4);
        "#,
        election_id,
        query.region,
        query.party,
        query.primary_vote
    )
    .fetch_all(&ctx.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut region_map: HashMap<i64, RegionVotes> = HashMap::new();
    for row in result {
        //get or set empty region
        let region = region_map
            .entry(row.region_id)
            .or_insert_with(|| RegionVotes {
                region: Region {
                    id: row.region_id,
                    name: row.region_name,
                },
                turnout: vec![],
                votes: vec![],
            });

        //add votes
        if let Some(party_id) = row.party_id {
            region.votes.push(ElectoralVote {
                party: Party {
                    id: party_id,
                    name: row.party_name.unwrap(),
                },
                votes: row.votes.unwrap_or(0),
                primary_vote: row.primary_vote.unwrap_or(false),
            });
        }

        if let Some(eligible) = row.eligible {
            let turnout = VoteTurnout {
                eligible,
                voted: row.voted.unwrap_or(0),
                primary_vote: row.turnout_primary_vote.unwrap_or(false),
            };
            if !region.turnout.iter().any(|t| t == &turnout) {
                region.turnout.push(turnout);
            }
        }
    }

    let result: Vec<RegionVotes> = region_map.into_values().collect();

    Ok(Json(ElectionRegion {
        region: result,
        election,
    }))
}
