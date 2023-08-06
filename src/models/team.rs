use serde::{ Deserialize, Serialize };
use sqlx::FromRow;


#[derive(Clone, Debug, Deserialize, Serialize, FromRow)]
pub struct TeamEntity {
    pub id: i64,
    pub score: i16,
    pub table_entity_id: i64,
}

impl TeamEntity {
    pub fn new(id: i64, score: i16, table_entity_id: i64) -> Self {
        TeamEntity { id, score, table_entity_id }
    }

    pub fn get_id(self) -> i64 {
        self.id
    }
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NewTeamEntity {
    pub id: Option<i64>,
    pub score: i16,
    pub table_entity_id: i64,
}

impl NewTeamEntity {
    pub fn new(id: Option<i64>, score: i16, table_entity_id: i64) -> Self {
        NewTeamEntity { id, score, table_entity_id }
    }
}