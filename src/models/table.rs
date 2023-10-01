use serde::{ Deserialize, Serialize };
use sqlx::FromRow;


#[derive(Clone, Debug, Deserialize, Serialize, FromRow)]
pub struct TableEntity {
    pub id: i64,
    pub last_played: Option<i64>,
    pub maquina: Option<i64>,
    pub turn: i64,
    pub round_points: i16,
    pub winner_team: Option<i64>,
}

impl TableEntity {
    pub fn new(id: i64, last_played: Option<i64>, maquina: Option<i64>, turn: i64, round_points: i16, winner_team: Option<i64>) -> Self {
        TableEntity { id, last_played, maquina, turn, round_points, winner_team }
    }

    pub fn get_id(self) -> i64 {
        self.id
    }
}

impl PartialEq for TableEntity {
    
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.last_played == other.last_played && self.maquina == other.maquina
    }
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NewTableEntity {
    pub id: Option<i64>,
    pub last_played: Option<i64>,
    pub maquina: Option<i64>,
    pub turn: i64,
    pub round_points: i16,
    pub winner_team: Option<i64>,
}

impl NewTableEntity {
    pub fn new(id: Option<i64>, last_played: Option<i64>, maquina: Option<i64>, turn: i64, round_points: i16, winner_team: Option<i64>) -> Self {
        NewTableEntity { id, last_played, maquina, turn, round_points, winner_team }
    }
}
