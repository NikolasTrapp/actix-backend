use serde::{ Deserialize, Serialize };
use sqlx::FromRow;


#[derive(Clone, Copy, FromRow, Debug, )]
pub struct RoundEntity {
    pub player_id: i64,
    pub table_id: i64,
    pub card_id: i64,
    pub turn: i64,
}

impl RoundEntity {
    pub fn new (player_id: i64, table_id: i64, card_id: i64, turn: i64) -> Self {
        RoundEntity {player_id, table_id, card_id, turn, }
    }
}

