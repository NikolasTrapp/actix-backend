use sqlx::{ PgPool, Error };
use crate::models::table::{ TableEntity, NewTableEntity };


pub async fn get_all(pool: &PgPool) -> Result<Vec<TableEntity>, Error> {
    sqlx::query_as!(
        TableEntity,
        r#"SELECT id, last_played, maquina, turn, round_points, winner_team FROM tables_tb"#
    )
    .fetch_all(pool)
    .await
}

pub async fn get_by_id(table_id: i64, pool: &PgPool) -> Result<TableEntity, Error> {
    sqlx::query_as!(
        TableEntity,
        r#"
        SELECT     
        id, last_played, maquina, turn, round_points, winner_team
        FROM tables_tb 
        WHERE id = $1
        "#, 
        table_id,
    )
    .fetch_one(pool)
    .await
}

pub async fn save(new_table: &NewTableEntity, pool: &PgPool) -> Result<TableEntity, Error> {
    sqlx::query_as!(
        TableEntity,
        r#"
        INSERT INTO tables_tb (last_played, maquina, turn)
        VALUES ($1, $2, 1)
        RETURNING  id, last_played, maquina, turn, round_points, winner_team
        "#, 
        new_table.last_played,
        new_table.maquina,
    )
    .fetch_one(pool)
    .await
}

pub async fn delete(table_id: i64, pool: &PgPool) -> Result<i64, Error> {
    Ok(sqlx::query!(
        r#"
        DELETE FROM tables_tb
        WHERE id = $1
        RETURNING id
        "#,
        table_id
    )
    .fetch_one(pool)
    .await?
    .id)
}

pub async fn update(target_id: i64, table: TableEntity, pool: &PgPool) -> Result<TableEntity, Error> {
    sqlx::query_as!(
        TableEntity,
        r#"
        UPDATE tables_tb SET
        last_played = $1,
        maquina = $2
        WHERE id = $3
        RETURNING id, last_played, maquina, turn, round_points, winner_team
        "#, 
        table.last_played,
        table.maquina,
        target_id,
    )
    .fetch_one(pool)
    .await
}

pub async fn set_table_maquina(target_id: i64, card_id: i64, pool: &PgPool) -> Result<TableEntity, Error> {
    sqlx::query_as!(
        TableEntity,
        r#"
        UPDATE tables_tb SET
        maquina = $1
        WHERE id = $2
        RETURNING id, last_played, maquina, turn, round_points, winner_team
        "#, 
        card_id,
        target_id,
    )
    .fetch_one(pool)
    .await
}

pub async fn add_turn(target_id: i64, pool: &PgPool) -> Result<TableEntity, Error> {
    sqlx::query_as!(
        TableEntity,
        r#"
        UPDATE tables_tb SET
        turn = turn + 1
        WHERE id = $1
        RETURNING id, last_played, maquina, turn, round_points, winner_team
        "#, 
        target_id,
    )
    .fetch_one(pool)
    .await
}


