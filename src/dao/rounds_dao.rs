use sqlx::{ PgPool, Error };
use crate::models::{round::RoundEntity, card::CardEntity, suit::Suit, table::TableEntity};


pub async fn get_cards_by_table_id(table_id: i64, pool: &PgPool) -> Result<Vec<CardEntity>, Error> {
    sqlx::query_as!(
        CardEntity,
        r#"
        SELECT 
            card.id, 
            suit AS "suit: Suit", 
            card_value, 
            is_manilha, 
            player_entity_id
        FROM cards_tb card
            INNER JOIN tables_tb tt ON tt.id = $1
            INNER JOIN rounds_tb round ON round.card_id = card.id and round.table_id = tt.id and round.turn = tt.turn
        "#,
        table_id
    ).fetch_all(pool)
    .await
}

pub async fn add_played_card(card_id: i64, player_id: i64, table: TableEntity, pool: &PgPool) -> Result<RoundEntity, Error> {
    sqlx::query_as!(
        RoundEntity,
        r#"
        INSERT INTO rounds_tb (table_id, player_id, card_id, turn)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#,
        table.id,
        player_id,
        card_id,
        table.turn,      
    ).fetch_one(pool)
    .await
}