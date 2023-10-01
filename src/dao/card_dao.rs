use sqlx::{ PgPool, Error};
use crate::models::card::{ CardEntity, NewCardEntity };
use crate::models::suit::Suit;
use crate::models::player::PlayerEntity;

pub async fn get_all(pool: &PgPool) -> Result<Vec<CardEntity>, Error> {
    sqlx::query_as!(
        CardEntity,
        r#"SELECT id, suit AS "suit: Suit", card_value, is_manilha, player_entity_id FROM cards_tb"#
    )
    .fetch_all(pool)
    .await
}

pub async fn get_by_id(card_id: i64, pool: &PgPool) -> Result<CardEntity, Error> {
    sqlx::query_as!(
        CardEntity,
        r#"
        SELECT id, suit AS "suit: Suit", card_value, is_manilha, player_entity_id 
        FROM cards_tb 
        WHERE id = $1
        "#, 
        card_id,
    )
    .fetch_one(pool)
    .await
}

pub async fn save(new_card: NewCardEntity, pool: &PgPool) -> Result<CardEntity, Error> {
    sqlx::query_as!(
        CardEntity,
        r#"
        INSERT INTO cards_tb (suit, card_value, is_manilha, player_entity_id)
        VALUES ($1, $2, $3, $4)
        RETURNING id, suit AS "suit: Suit", card_value, is_manilha, player_entity_id
        "#, 
        new_card.suit as Suit,
        new_card.card_value,
        new_card.is_manilha,
        new_card.player_entity_id,
    )
    .fetch_one(pool)
    .await
}

///Feito na preguiça 🤡
pub async fn save_cards(new_cards: Vec<NewCardEntity>, pool: &PgPool) -> Result<Vec<CardEntity>, Error> {
    sqlx::query_as!(
        CardEntity,
        r#"
        INSERT INTO cards_tb (suit, card_value, is_manilha, player_entity_id)
        VALUES ($1, $2, $3, $4),
        ($5, $6, $7, $8),
        ($9, $10, $11, $12)
        RETURNING id, suit AS "suit: Suit", card_value, is_manilha, player_entity_id
        "#, 
        new_cards[0].suit as Suit,
        new_cards[0].card_value,
        new_cards[0].is_manilha,
        new_cards[0].player_entity_id,
        new_cards[1].suit as Suit,
        new_cards[1].card_value,
        new_cards[1].is_manilha,
        new_cards[1].player_entity_id,
        new_cards[2].suit as Suit,
        new_cards[2].card_value,
        new_cards[2].is_manilha,
        new_cards[2].player_entity_id,
    )
    .fetch_all(pool)
    .await
}

pub async fn delete(card_id: i64, pool: &PgPool) -> Result<i64, Error> {
    Ok(sqlx::query!(
        r#"
        DELETE FROM cards_tb
        WHERE id = $1
        RETURNING id
        "#,
        card_id
    )
    .fetch_one(pool)
    .await?
    .id)
}

pub async fn update(target_id: i64, card: CardEntity, pool: &PgPool) -> Result<CardEntity, Error> {
    sqlx::query_as!(
        CardEntity,
        r#"
        UPDATE cards_tb SET
        suit = $1,
        card_value = $2,
        is_manilha = $3,
        player_entity_id = $4
        WHERE id = $5
        RETURNING id, suit AS "suit: Suit", card_value, is_manilha, player_entity_id
        "#, 
        card.suit as Suit,
        card.card_value,
        card.is_manilha,
        card.player_entity_id,
        target_id,
    )
    .fetch_one(pool)
    .await
}

pub async fn get_player_cards(player_id: i64, pool: &PgPool) -> Result<Vec<CardEntity>, Error> {
    sqlx::query_as!(
        CardEntity,
        r#"
        SELECT id, suit AS "suit: Suit", card_value, is_manilha, player_entity_id 
        FROM cards_tb 
        WHERE player_entity_id = $1
        "#,
        player_id
    ).fetch_all(pool)
    .await
}

pub async fn delete_player_cards(player_id: i64, pool: &PgPool) -> Result<i64, Error> {
    let mut count = 0;
    let _ = sqlx::query!(
        "
        DELETE FROM cards_tb
        WHERE player_entity_id = $1
        ",
        player_id
    )
    .execute(pool)
    .await
    .map(|_| count += 1 );

    Ok(count)
}

pub async fn delete_cards_by_id(cards: &Vec<CardEntity>, pool: &PgPool) -> Result<i64, Error> {
    let mut count = 0;
    for card in cards {
        let _ = sqlx::query!(
            r#"
            DELETE FROM cards_tb
            WHERE id = $1
            "#, card.id
        ).execute(pool)
        .await;

        count += 1;
    }

    Ok(count)
}

pub async fn get_cards_by_players_id_list(players_id: Vec<i64>, pool: &PgPool) -> Result<Vec<CardEntity>, Error> {

    //let ids = players_id.into_iter().map(|player| player.id.to_string()).collect::<String>();

    sqlx::query_as!(
        CardEntity,
        r#"
        SELECT id, suit AS "suit: Suit", card_value, is_manilha, player_entity_id FROM cards_tb
        WHERE player_entity_id in ($1, $2, $3, $4)
        "#,
        players_id[0],
        players_id[1],
        players_id[2],
        players_id[3],
        ).fetch_all(pool)
        .await
}
