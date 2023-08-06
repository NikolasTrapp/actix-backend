use sqlx::{ PgPool, Error };
use crate::models::player::{ PlayerEntity, NewPlayerEntity, };


pub async fn get_all(pool: &PgPool) -> Result<Vec<PlayerEntity>, Error> {
    sqlx::query_as!(
        PlayerEntity,
        r#"SELECT id, name, victories, team_entity_id FROM players_tb"#
    )
    .fetch_all(pool)
    .await
}

pub async fn get_by_id(player_id: i64, pool: &PgPool) -> Result<PlayerEntity, Error> {
    sqlx::query_as!(
        PlayerEntity,
        r#"
        SELECT id, name, victories, team_entity_id 
        FROM players_tb 
        WHERE id = $1
        "#, 
        player_id,
    )
    .fetch_one(pool)
    .await
}

pub async fn save(new_player: NewPlayerEntity, pool: &PgPool) -> Result<PlayerEntity, Error> {
    sqlx::query_as!(
        PlayerEntity,
        r#"
        INSERT INTO players_tb (name, victories, team_entity_id)
        VALUES ($1, $2, $3)
        RETURNING *
        "#, 
        new_player.name,
        new_player.victories,
        new_player.team_entity_id,
    )
    .fetch_one(pool)
    .await
}

pub async fn delete(player_id: i64, pool: &PgPool) -> Result<i64, Error> {
    Ok(sqlx::query!(
        r#"
        DELETE FROM players_tb
        WHERE id = $1
        RETURNING id
        "#,
        player_id
    )
    .fetch_one(pool)
    .await?
    .id)
}

pub async fn update(target_id: i64, player: PlayerEntity, pool: &PgPool) -> Result<PlayerEntity, Error> {
    sqlx::query_as!(
        PlayerEntity,
        r#"
        UPDATE players_tb SET
        name = $1,
        victories = $2,
        team_entity_id = $3
        WHERE id = $4
        RETURNING *
        "#, 
        player.name,
        player.victories,
        player.team_entity_id,
        target_id,
    )
    .fetch_one(pool)
    .await
}

pub async fn get_team_players(team_id: i64, pool: &PgPool) -> Result<Vec<PlayerEntity>, Error> {
    sqlx::query_as!(
        PlayerEntity,
        r#"
        SELECT id, name, victories, team_entity_id 
        FROM players_tb 
        WHERE id = $1
        "#, 
        team_id,
    )
    .fetch_all(pool)
    .await
}

pub async fn add_player_to_team(player_id: i64, team_id: i64, pool: &PgPool) -> Result<PlayerEntity, Error> {
    sqlx::query_as!(
        PlayerEntity,
        r#"
        UPDATE players_tb SET 
        team_entity_id = $2
        WHERE id = $1
        RETURNING *
        "#,
        player_id,
        team_id
    ).fetch_one(pool)
    .await
}