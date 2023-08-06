use serde::{ Deserialize, Serialize };
use sqlx::FromRow;


#[derive(Clone, Debug, Deserialize, Serialize, FromRow)]
pub struct TableEntity {
    pub id: i64,
    pub last_played: Option<i64>,
    pub manilha: Option<i64>,
}

impl TableEntity {
    pub fn new(id: i64, last_played: Option<i64>, manilha: Option<i64>) -> Self {
        TableEntity { id, last_played, manilha }
    }

    pub fn get_id(self) -> i64 {
        self.id
    }
}

impl PartialEq for TableEntity {
    
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.last_played == other.last_played && self.manilha == other.manilha
    }
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NewTableEntity {
    pub id: Option<i64>,
    pub last_played: Option<i64>,
    pub manilha: Option<i64>,
}

impl NewTableEntity {
    pub fn new(id: Option<i64>, last_played: Option<i64>, manilha: Option<i64>) -> Self {
        NewTableEntity { id, last_played, manilha }
    }
}