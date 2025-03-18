use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Party{
    pub id: i64,
    pub name: String,
    pub abbreviation: String
}

#[derive(Debug, Serialize)]
pub struct Region{
    pub id: i64,
    pub num: i64,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct Election {
    pub id: i64,
    pub date: i64,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct Turnout {
    pub election_id: i64,
    pub region_id: i64,
    pub eligible: i64,
    pub voted: i64,
    pub primary_vote: bool
}

#[derive(Debug, Serialize)]
pub struct Vote {
    pub id: i64,   
    pub election_id: i64,
    pub region_id: i64,
    pub party_id: i64,
    pub votes: i64,
    pub primary_vote: bool
}
