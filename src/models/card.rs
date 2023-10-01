use super::{suit::Suit, projections::CardProjection};
use serde::{ Deserialize, Serialize };
use sqlx::FromRow;

pub trait CardFunctions {
    fn card_value(&self) -> i16;
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, FromRow)]
pub struct CardEntity {
    pub id: i64,
    pub suit: Suit,
    pub card_value: i16,
    pub is_manilha: bool,
    pub player_entity_id: Option<i64>,
}

impl CardEntity {
    pub fn new(id: i64, suit: Suit, card_value: i16, is_manilha: bool, player_entity_id: Option<i64>) -> Self {
        CardEntity { id, suit, card_value, is_manilha, player_entity_id }
    }

    pub fn get_id(self) -> i64 {
        self.id
    }

    pub fn get_real_card_strength(self) -> u8 {
        let mut card_strength: u8 = match self.card_value {
            4 => 1,
            5 => 2,
            6 => 3,
            7 => 4,
            10 => 5,
            11 => 6,
            12 => 7,
            1 => 8,
            2 => 9,
            3 => 10,
            _ => panic!("Card value {} doesn't exist.", self.card_value)
        };

        if self.is_manilha {
            card_strength += Suit::get_suit_strength(&self.suit);
        }

        card_strength
    }
}


#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct NewCardEntity {
    pub id: Option<i64>,
    pub suit: Suit,
    pub card_value: i16,
    pub is_manilha: bool,
    pub player_entity_id: Option<i64>,
}

impl NewCardEntity {
    pub fn new(id: Option<i64>, suit: Suit, card_value: i16, is_manilha: bool, player_entity_id: Option<i64>) -> Self {
        NewCardEntity { id, suit, card_value, is_manilha, player_entity_id, }
    }

    pub fn get_cards_list() -> Vec<NewCardEntity> {
        vec![
            NewCardEntity::new(None, Suit::Diamonds, 1, false, None),
            NewCardEntity::new(None, Suit::Diamonds, 2, false, None),
            NewCardEntity::new(None, Suit::Diamonds, 3, false, None),
            NewCardEntity::new(None, Suit::Diamonds, 4, false, None),
            NewCardEntity::new(None, Suit::Diamonds, 5, false, None),
            NewCardEntity::new(None, Suit::Diamonds, 6, false, None),
            NewCardEntity::new(None, Suit::Diamonds, 7, false, None),
            NewCardEntity::new(None, Suit::Diamonds, 10, false, None),
            NewCardEntity::new(None, Suit::Diamonds, 11, false, None),
            NewCardEntity::new(None, Suit::Diamonds, 12, false, None),
            NewCardEntity::new(None, Suit::Spades, 1, false, None),
            NewCardEntity::new(None, Suit::Spades, 2, false, None),
            NewCardEntity::new(None, Suit::Spades, 3, false, None),
            NewCardEntity::new(None, Suit::Spades, 4, false, None),
            NewCardEntity::new(None, Suit::Spades, 5, false, None),
            NewCardEntity::new(None, Suit::Spades, 6, false, None),
            NewCardEntity::new(None, Suit::Spades, 7, false, None),
            NewCardEntity::new(None, Suit::Spades, 10, false, None),
            NewCardEntity::new(None, Suit::Spades, 11, false, None),
            NewCardEntity::new(None, Suit::Spades, 12, false, None),
            NewCardEntity::new(None, Suit::Clubs, 1, false, None),
            NewCardEntity::new(None, Suit::Clubs, 2, false, None),
            NewCardEntity::new(None, Suit::Clubs, 3, false, None),
            NewCardEntity::new(None, Suit::Clubs, 4, false, None),
            NewCardEntity::new(None, Suit::Clubs, 5, false, None),
            NewCardEntity::new(None, Suit::Clubs, 6, false, None),
            NewCardEntity::new(None, Suit::Clubs, 7, false, None),
            NewCardEntity::new(None, Suit::Clubs, 10, false, None),
            NewCardEntity::new(None, Suit::Clubs, 11, false, None),
            NewCardEntity::new(None, Suit::Clubs, 12, false, None),
            NewCardEntity::new(None, Suit::Hearts, 1, false, None),
            NewCardEntity::new(None, Suit::Hearts, 2, false, None),
            NewCardEntity::new(None, Suit::Hearts, 3, false, None),
            NewCardEntity::new(None, Suit::Hearts, 4, false, None),
            NewCardEntity::new(None, Suit::Hearts, 5, false, None),
            NewCardEntity::new(None, Suit::Hearts, 6, false, None),
            NewCardEntity::new(None, Suit::Hearts, 7, false, None),
            NewCardEntity::new(None, Suit::Hearts, 10, false, None),
            NewCardEntity::new(None, Suit::Hearts, 11, false, None),
            NewCardEntity::new(None, Suit::Hearts, 12, false, None),
        ]
    }

    pub fn to_entity_manilha(&self) -> CardProjection {
        CardProjection::new(self.id.unwrap(), self.suit, self.card_value, self.is_manilha)
    }
}



impl CardFunctions for NewCardEntity {
    fn card_value(&self) -> i16 {
        self.card_value
    }
}

impl CardFunctions for CardEntity {
    fn card_value(&self) -> i16 {
        self.card_value
    }
}