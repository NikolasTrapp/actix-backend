use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Type, Copy)]
#[sqlx(type_name = "suit")]
pub enum Suit {
    Clubs,
    Spades,
    Hearts,
    Diamonds,
}

impl Suit {
    pub fn get_suit_strength(suit: &Suit) -> u8 {
        match suit {
            Suit::Diamonds => 1,
            Suit::Spades => 2,
            Suit::Hearts => 3,
            Suit::Clubs => 4,
        }
    } 
}