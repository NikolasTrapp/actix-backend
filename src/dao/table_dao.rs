use sqlx::{ PgPool, Error };
use crate::models::table::{ TableEntity, NewTableEntity };


pub async fn get_all(pool: &PgPool) -> Result<Vec<TableEntity>, Error> {
    sqlx::query_as!(
        TableEntity,
        r#"SELECT id, last_played, manilha FROM tables_tb"#
    )
    .fetch_all(pool)
    .await
}

pub async fn get_by_id(table_id: i64, pool: &PgPool) -> Result<TableEntity, Error> {
    sqlx::query_as!(
        TableEntity,
        r#"
        SELECT id, last_played, manilha 
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
        INSERT INTO tables_tb (last_played, manilha)
        VALUES ($1, $2)
        RETURNING *
        "#, 
        new_table.last_played,
        new_table.manilha,
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
        manilha = $2
        WHERE id = $3
        RETURNING *
        "#, 
        table.last_played,
        table.manilha,
        target_id,
    )
    .fetch_one(pool)
    .await
}
