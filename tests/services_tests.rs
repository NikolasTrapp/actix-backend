#[cfg(test)]
mod basic_sql_operation_tests {

    use actix_backend::dao::table_dao::*;
    use actix_backend::models::table::*;
    use sqlx::PgPool;
    use sqlx::postgres::{PgConnectOptions, PgQueryResult};
    use actix_backend::utils;


    pub async fn create_test_pool() -> Result<PgPool, sqlx::Error> {
        let options = PgConnectOptions::new()
            .host("localhost")
            .port(5432)
            .database("testes_db")
            .username("postgres")
            .password("postgres");

            PgPool::connect_with(options).await
    }

    pub async fn truncate_table(conn: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query("TRUNCATE TABLE tables_tb, cards_tb, players_tb, teams_tb RESTART IDENTITY").execute(conn).await
    }

    pub async fn add_table_to_tests(conn: &PgPool) -> Result<Vec<TableEntity>, sqlx::Error> {
        sqlx::query_as!(
            TableEntity,
            r#"
            INSERT INTO tables_tb (last_played, manilha)
            VALUES (1, 1), (1, 2), (2, 1), (2, 2)
            RETURNING *
            "#
        ).fetch_all(conn)
        .await
    
    }
    
    #[tokio::test]
    async fn get_all_test_one() -> Result<(), sqlx::Error>{
        let conn = create_test_pool().await?;
        let data = get_all(&conn).await?;
        Ok(assert_eq!(Vec::<TableEntity>::new(), data))
    }

    #[tokio::test]
    async fn get_all_test_two() -> Result<(), sqlx::Error>{
        let conn = create_test_pool().await?;
        let mocked_tables = add_table_to_tests(&conn).await?;
        let data = get_all(&conn).await?;
        let _ = truncate_table(&conn).await;
        Ok(assert_eq!(mocked_tables, data))
    }

    #[tokio::test]
    async fn get_all_test_three() -> Result<(), sqlx::Error>{
        let conn = create_test_pool().await?;
        let mocked_tables = vec![
            TableEntity::new(99, Some(99), Some(99))
        ];
        let data = get_all(&conn).await?;
        let _ = truncate_table(&conn).await;
        Ok(assert_ne!(mocked_tables, data))
    }

    #[tokio::test]
    async fn save_table_test() -> Result<(), sqlx::Error>{
        let conn = create_test_pool().await?;
        let t = TableEntity::new(1, Some(1), Some(1));
        let table = save(&NewTableEntity::new(None, Some(1), Some(1)), &conn).await?;

        let _ = truncate_table(&conn).await;
        Ok(assert_eq!(t, table))
    }

    #[tokio::test]
    async fn find_by_id_test_one() -> Result<(), sqlx::Error>{
        let conn = create_test_pool().await?;
        let compare_with = TableEntity {
            id: 1,
            last_played: Some(1),
            manilha: Some(1),
        };
        let _ = add_table_to_tests(&conn).await?;

        let queried_table = get_by_id(1, &conn).await?;

        let _ = truncate_table(&conn).await;
        Ok(assert_eq!(queried_table, compare_with))
    }

    #[tokio::test]
    async fn find_by_id_test_two() -> Result<(), sqlx::Error>{
        let conn = create_test_pool().await?;
        let compare_with = TableEntity {
            id: 1,
            last_played: Some(1),
            manilha: Some(1),
        };
        let _ = add_table_to_tests(&conn).await?;

        let queried_table = get_by_id(99, &conn).await?;

        let _ = truncate_table(&conn).await;
        Ok(assert_ne!(queried_table, compare_with))
    }

    #[test]
    fn get_manilhas() {
        let cards = utils::CardsShuffler::new().get_shuffled_cards();

        for i in cards {
            for j in i {
                println!("{:?}", j);
            }
        }
    }
}