use serde::{Deserialize, Serialize};
use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{self, IsNull, Output, ToSql};
use std::fmt::Display;
use std::io::Write;


#[derive(Queryable, Selectable, Identifiable, Associations, AsChangeset, Clone, Debug, Deserialize, Serialize)]
#[diesel(primary_key(id))]
#[diesel(belongs_to(PlayerEntity))]
#[diesel(table_name = crate::schema::cards_tb)]
pub struct CardEntity {
    pub id: i64,
    pub suit: Suit,
    pub card_value: i16,
    pub is_manilha: bool,
    pub player_entity_id: i64,
}

impl CardEntity {
    pub fn new(id: i64, suit: Suit, card_value: i16, is_manilha: bool, player_entity_id: i64) -> Self {
        CardEntity { id, suit, card_value, is_manilha, player_entity_id }
    }

    pub fn get_id(self) -> i64 {
        self.id
    }

    pub fn get_real_card_strength(self, other: CardEntity) -> u8 {
        let mut card_strength: u8 = match other.card_value {
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
            _ => panic!("Card value {} doesn't exist.", other.card_value)
        };

        if other.is_manilha {
            card_strength += Suit::get_suit_strength(&other.suit);
        }

        card_strength
    }
}

impl PartialEq for CardEntity {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id &&
        self.suit == other.suit &&
        self.card_value == other.card_value &&
        self.is_manilha == other.is_manilha &&
        self.player_entity_id == other.player_entity_id 
    }
}


#[derive(Insertable, PartialEq)]
#[diesel(table_name = crate::schema::cards_tb)]
pub struct NewCardEntity {
    pub id: Option<i64>,
    pub suit: Suit,
    pub card_value: i16,
    pub is_manilha: bool,
    pub player_entity_id: i64,
}

impl NewCardEntity {
    pub fn new(id: Option<i64>, suit: Suit, card_value: i16, is_manilha: bool, player_entity_id: i64) -> Self {
        NewCardEntity { id, suit, card_value, is_manilha, player_entity_id, }
    }
}


#[derive(Queryable, Selectable, Identifiable, Associations, AsChangeset, Clone, Debug, Deserialize, Serialize)]
#[diesel(primary_key(id))]
#[diesel(belongs_to(TeamEntity))]
#[diesel(table_name = crate::schema::players_tb)]
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

impl PartialEq for PlayerEntity {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id &&
        self.name == other.name &&
        self.victories == other.victories &&
        self.team_entity_id == other.team_entity_id
    }
}


#[derive(Insertable)]
#[diesel(table_name = crate::schema::players_tb)]
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

#[derive(Queryable, Selectable, Identifiable, Associations, AsChangeset, Clone, Debug, Deserialize, Serialize)]
#[diesel(belongs_to(TableEntity))]
#[diesel(table_name = crate::schema::teams_tb)]
#[diesel(primary_key(id))]
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

impl PartialEq for TeamEntity {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id &&
        self.score == other.score &&
        self.table_entity_id == other.table_entity_id
    }
}


#[derive(Insertable)]
#[diesel(table_name = crate::schema::teams_tb)]
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

#[derive(Queryable, Identifiable, Selectable, Debug, Clone, AsChangeset)]
#[diesel(table_name = crate::schema::tables_tb)]
#[diesel(primary_key(id))]
pub struct TableEntity {
    pub id: i64,
    pub last_played: Option<i64>,
}

impl TableEntity {
    pub fn new(id: i64, last_played: Option<i64>) -> Self {
        TableEntity { id, last_played }
    }

    pub fn get_id(self) -> i64 {
        self.id
    }
}

impl PartialEq for TableEntity {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id &&
        self.last_played == other.last_played
    }
}


#[derive(Insertable)]
#[diesel(table_name = crate::schema::tables_tb)]
pub struct NewTableEntity {
    pub id: Option<i64>,
    pub last_played: Option<i64>,
}

impl NewTableEntity {
    pub fn new(id: Option<i64>, last_played: Option<i64>) -> Self {
        NewTableEntity { id, last_played }
    }
}


#[derive(Clone, Debug, AsExpression, FromSqlRow, Serialize, Deserialize )]
#[diesel(sql_type = crate::schema::sql_types::Suit)]
pub enum Suit {
    Clubs,
    Spades,
    Hearts,
    Diamonds,
    Blocked(String),
}

impl PartialEq for Suit {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Suit::Clubs, Suit::Clubs)
            | (Suit::Spades, Suit::Spades)
            | (Suit::Diamonds, Suit::Diamonds)
            | (Suit::Hearts, Suit::Hearts) => true,
            (Suit::Blocked(data1), Suit::Blocked(data2)) => data1 == data2,
            _ => false,
        }
    }
}

impl ToSql<crate::schema::sql_types::Suit, Pg> for Suit {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match self {
            Suit::Clubs => out.write_all(b"Clubs")?,
            Suit::Spades => out.write_all(b"Spades")?,
            Suit::Hearts => out.write_all(b"Hearts")?,
            Suit::Diamonds => out.write_all(b"Diamonds")?,
            Suit::Blocked(_) => out.write_all(b"not_found")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<crate::schema::sql_types::Suit, Pg> for Suit {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"Clubs" => Ok(Suit::Clubs),
            b"Spades" => Ok(Suit::Spades),
            b"Hearts" => Ok(Suit::Hearts),
            b"Diamonds" => Ok(Suit::Diamonds),
            b"Not_Found" => Ok(Suit::Blocked(String::from("Not_Fount"))),
            _ => Err("Enum nao identificada".into()),
        }
    }
}

impl Suit {
    pub fn get_suit_strength(suit: &Suit) -> u8 {
        match suit {
            Suit::Diamonds => 1,
            Suit::Spades => 2,
            Suit::Hearts => 3,
            Suit::Clubs => 4,
            Suit::Blocked(_) => 0,
        }
    } 
}