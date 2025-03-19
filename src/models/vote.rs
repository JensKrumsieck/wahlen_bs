use super::party::Party;
use crate::AppContext;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Vote {
    pub id: i64,
    pub election_id: i64,
    pub region_id: i64,
    pub party_id: i64,
    pub votes: i64,
    pub primary_vote: bool,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct VoteTurnout {    
    #[schema(example = 69420)]
    pub eligible: i64,
    #[schema(example = 42069)]
    pub voted: i64,    
    pub primary_vote: bool,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub(crate) struct ElectoralVote {
    #[serde(flatten)]
    pub party: Party,
    #[schema(example = 161)]
    pub votes: i64,
    pub primary_vote: bool,
}

pub(crate) async fn fetch_votes(
    election_id: i64,
    region_id: i64,
    ctx: &AppContext,
) -> Result<Vec<ElectoralVote>, sqlx::Error> {
    let votes = sqlx::query!(
        "SELECT v.id, v.votes, v.primary_vote, 
        p.id as party_id,
        p.name as party_name,
        p.abbreviation as party_abbreviation
        FROM vote v        
        JOIN party p ON v.party_id = p.id
        WHERE election_id = $1 AND region_id = $2",
        election_id,
        region_id,
    )
    .fetch_all(&ctx.db)
    .await?;

    Ok(votes
        .into_iter()
        .map(|v| ElectoralVote {
            party: Party {
                id: v.party_id,
                name: v.party_name,
                abbreviation: v.party_abbreviation,
            },
            votes: v.votes,
            primary_vote: v.primary_vote,
        })
        .collect::<Vec<_>>())
}
