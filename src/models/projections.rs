use super::{suit::Suit, card::CardEntity};
use serde::{ Deserialize, Serialize };

#[derive(Serialize)]
pub struct TableProjection {
    id: i64,
    last_played: Option<i64>,
    maquina: Option<CardProjection>,
    teams: Vec<TeamProjection>,
    start: bool,
}

impl TableProjection {
    pub fn new(id: i64, last_played: Option<i64>, maquina: Option<CardProjection>, teams: Vec<TeamProjection>, start: bool ) -> Self {
        TableProjection { id, last_played, maquina, teams, start }
    }
}


#[derive(Serialize)]
pub struct TeamProjection {
    id: i64,
    score: i16,
    players: Vec<PlayerProjection>,
}

impl TeamProjection {
    pub fn new (id: i64, score: i16, players: Vec<PlayerProjection> ) -> Self {
        TeamProjection { id, score, players }
    }
}


#[derive(Serialize)]
pub struct PlayerProjection {
    id: i64,
    name: String,
    cards: Vec<CardProjection>,
}

impl PlayerProjection {
    pub fn new(id: i64, name: String, cards: Vec<CardProjection>) -> Self {
        PlayerProjection { id, name, cards }
    }
}


#[derive(Serialize)]
pub struct CardProjection {
    id: i64,
    suit: Suit,
    card_value: i16,
    is_manilha: bool,
}

impl CardProjection {
    pub fn new(id: i64, suit: Suit, card_value: i16, is_manilha: bool) -> Self {
        CardProjection { id, suit, card_value, is_manilha }
    }

    pub fn from_card(src: CardEntity) -> Self {
        CardProjection { id: src.id, suit: src.suit, card_value: src.card_value, is_manilha: src.is_manilha }
    }
}

trait CardFunctions {
    fn card_value(&self) -> i16;
}

impl CardFunctions for CardEntity {
    fn card_value(&self) -> i16 {
        self.card_value
    }
}