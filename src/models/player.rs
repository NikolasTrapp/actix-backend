use serde::{ Deserialize, Serialize };
use sqlx::FromRow;


#[derive(Clone, Debug, Deserialize, Serialize, FromRow)]
pub struct PlayerEntity {
    pub id: i64,
    pub name: String,
    pub victories: i64,
    pub team_entity_id: i64,
}

impl PlayerEntity {
    pub fn new(id: i64, name: String, victories: i64, team_entity_id: i64) -> Self {
        PlayerEntity {
            id,
            name,
            victories,
            team_entity_id,
        }
    }

    pub fn get_id(self) -> i64 {
        self.id
    }
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NewPlayerEntity {
    pub id: Option<i64>,
    pub name: String,
    pub victories: i64,
    pub team_entity_id: i64,
}

impl NewPlayerEntity {
    pub fn new(id: Option<i64>, name: String, victories: i64, team_entity_id: i64) -> Self {
        NewPlayerEntity { id, name, victories, team_entity_id, }
    }
}