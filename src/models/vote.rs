use super::party::Party;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Vote {
    pub id: i64,
    pub election_id: i64,
    pub region_id: i64,
    pub party_id: String,
    pub votes: i64,
    pub primary_vote: bool,
}

#[derive(Debug, Serialize, utoipa::ToSchema, PartialEq)]
pub struct VoteTurnout {
    #[schema(example = 69420)]
    pub eligible: i64,
    #[schema(example = 42069)]
    pub voted: i64,
    pub primary_vote: bool,
    #[schema(example = 0.69)]
    pub turnout: f32,
}

#[derive(Debug, Serialize, utoipa::ToSchema, PartialEq)]
pub(crate) struct ElectoralVote {
    #[serde(flatten)]
    pub party: Party,
    #[schema(example = 161)]
    pub votes: i64,
    pub primary_vote: bool,
    #[schema(example = 0.51)]
    pub percentage: f32,
}
