// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "suit"))]
    pub struct Suit;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Suit;

    cards_tb (id) {
        id -> Int8,
        suit -> Suit,
        card_value -> Int2,
        is_manilha -> Bool,
        player_entity_id -> Int8,
    }
}

diesel::table! {
    players_tb (id) {
        id -> Int8,
        name -> Varchar,
        victories -> Int8,
        team_entity_id -> Int8,
    }
}

diesel::table! {
    tables_tb (id) {
        id -> Int8,
        last_played -> Nullable<Int8>,
    }
}

diesel::table! {
    teams_tb (id) {
        id -> Int8,
        score -> Int2,
        table_entity_id -> Int8,
    }
}

diesel::joinable!(cards_tb -> players_tb (player_entity_id));
diesel::joinable!(players_tb -> teams_tb (team_entity_id));
diesel::joinable!(teams_tb -> tables_tb (table_entity_id));

diesel::allow_tables_to_appear_in_same_query!(
    cards_tb,
    players_tb,
    tables_tb,
    teams_tb,
);
