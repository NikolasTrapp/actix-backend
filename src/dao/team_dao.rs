use sqlx::{ PgPool, Error };
use crate::models::team::{ TeamEntity, NewTeamEntity };


pub async fn get_all(pool: &PgPool) -> Result<Vec<TeamEntity>, Error> {
    sqlx::query_as!(
        TeamEntity,
        r#"SELECT id, score, table_entity_id FROM teams_tb"#
    )
    .fetch_all(pool)
    .await
}

pub async fn get_by_id(team_id: i64, pool: &PgPool) -> Result<TeamEntity, Error> {
    sqlx::query_as!(
        TeamEntity,
        r#"
        SELECT id, score, table_entity_id 
        FROM teams_tb 
        WHERE id = $1
        "#, 
        team_id,
    )
    .fetch_one(pool)
    .await
}

pub async fn save(new_team: NewTeamEntity, pool: &PgPool) -> Result<TeamEntity, Error> {
    sqlx::query_as!(
        TeamEntity,
        r#"
        INSERT INTO teams_tb (score, table_entity_id)
        VALUES ($1, $2)
        RETURNING *
        "#, 
        new_team.score,
        new_team.table_entity_id,
    )
    .fetch_one(pool)
    .await
}

pub async fn delete(team_id: i64, pool: &PgPool) -> Result<i64, Error> {
    Ok(sqlx::query!(
        r#"
        DELETE FROM teams_tb
        WHERE id = $1
        RETURNING id
        "#,
        team_id
    )
    .fetch_one(pool)
    .await?
    .id)
}

pub async fn update(target_id: i64, team: TeamEntity, pool: &PgPool) -> Result<TeamEntity, Error> {
    sqlx::query_as!(
        TeamEntity,
        r#"
        UPDATE teams_tb SET
        score = $1,
        table_entity_id = $2
        WHERE id = $3
        RETURNING *
        "#, 
        team.score,
        team.table_entity_id,
        target_id,
    )
    .fetch_one(pool)
    .await
}

pub async fn get_table_teams(table_id: i64, pool: &PgPool) -> Result<Vec<TeamEntity>, Error> {
    sqlx::query_as!(
        TeamEntity,
        r#"
        SELECT id, score, table_entity_id 
        FROM teams_tb 
        WHERE id = $1
        LIMIT 2
        "#, 
        table_id,
    )
    .fetch_all(pool)
    .await
}

pub async fn create_two_empty_teams(table_id: i64, pool: &PgPool) -> Result<Vec<TeamEntity>, Error> {
    sqlx::query_as!(
        TeamEntity,
        r#"
        INSERT INTO teams_tb (score, table_entity_id)
        VALUES (0, $1), (0, $1)
        RETURNING *
        "#,
        table_id
    ).fetch_all(pool)
    .await
}