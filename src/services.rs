pub mod cards_dao {
    use crate::schema::cards_tb::dsl::cards_tb;
    use crate::schema::cards_tb as primary_cards_table;
    use primary_cards_table::{id, player_entity_id};
    use crate::models::{ CardEntity, NewCardEntity };
    use crate::db;
    use diesel::prelude::*;
    use diesel::QueryDsl;


    pub fn get_all() -> Vec<CardEntity> {
        let mut conn = db::establish_connection();
        let results = cards_tb.load::<CardEntity>(&mut conn)
            .expect("Ou tu é muito burro ou eu ratiei no código"); //TODO usar .optional() e criar um handler
        results
    }
    
    pub fn get_by_id(card_id: i64) -> Option<CardEntity> {
        let mut conn = db::establish_connection();
        let result = cards_tb
            .find(card_id)
            .first(&mut conn)
            .optional();

        match result {
            Ok(opt) => opt,
            Err(_) => None,
        }
    }

    pub fn get_player_cards(player_id: i64) -> Vec<CardEntity> {
        let mut conn = db::establish_connection();
        let results = cards_tb
            .filter(player_entity_id.eq(&player_id))
            .limit(3)
            .load::<CardEntity>(&mut conn);

        match results {
            Ok(x) => x,
            Err(err) => panic!("{}", err)
        }
            
    }
    
    pub fn save(card: &NewCardEntity) -> Option<CardEntity> {
        let mut conn = db::establish_connection();
        let new_card = diesel::insert_into(primary_cards_table::table)
            .values(card)
            .returning(CardEntity::as_returning())
            .get_result(&mut conn)
            .optional();

        match new_card {
            Ok(result) => result,
            Err(err) => panic!("Some error ocourred while trying to save card, {}", err),
        }
        
    }
    
    pub fn delete(card_id: i64) -> Result<i64, String> {
        let mut conn = db::establish_connection();
        let result = diesel::delete(cards_tb.filter(id.eq(&card_id))).execute(&mut conn).optional();
    
        match result {
            Ok(Some(_)) => Ok(card_id),
            Ok(None) => Err(format!("Any card found with id {}!", card_id)),
            Err(_) => panic!("An error occured while deleting card {}", card_id),
        }
    }
    
    pub fn update(card: CardEntity) -> CardEntity {
        let mut conn = db::establish_connection();
        match card.save_changes::<CardEntity>(&mut conn) {
            Ok(updated_card) => updated_card,
            Err(err) => panic!("Error updating card '{}' --> {}", card.get_id(), err),
        }
        
    }

    pub fn truncate() -> Result<(), diesel::result::Error> {
        println!("TRUNCATING TABLE!!!");

        let mut conn = db::establish_connection();
        diesel::delete(cards_tb)
            .execute(&mut conn)
            .map(|_| ())
    }
}


pub mod players_dao {
    
    use crate::schema::players_tb::dsl::players_tb;
    use crate::schema::players_tb::{self as primary_players_tb, team_entity_id, id };
    use crate::models::{ PlayerEntity, NewPlayerEntity };
    use crate::db;
    use diesel::prelude::*;
    use diesel::QueryDsl;



    pub fn get_all() -> Vec<PlayerEntity> {
        let mut conn = db::establish_connection();
        let results = players_tb.load::<PlayerEntity>(&mut conn)
            .expect("Ou tu é muito burro ou eu ratiei no código"); //TODO usar .optional() e criar um handler
        results
    }
    
    pub fn get_by_id(player_id: i64) -> Option<PlayerEntity> {
        let mut conn = db::establish_connection();
        let result = players_tb
            .filter(id.eq(&player_id))
            .first(&mut conn)
            .optional();

        match result {
            Ok(opt) => opt,
            Err(_) => None,
        }
    }

    pub fn get_players_by_team_id(team_id: i64) -> Vec<PlayerEntity> {
        let mut conn = db::establish_connection();
        let results = players_tb
            .filter(team_entity_id.eq(&team_id))
            .limit(2)
            .load::<PlayerEntity>(&mut conn);

        match results {
            Ok(x) => x,
            Err(err) => panic!("{}", err)
        }
            
    }
    
    pub fn save(player: &NewPlayerEntity) -> Option<PlayerEntity> {
        let mut conn = db::establish_connection();
        let new_player = diesel::insert_into(primary_players_tb::table)
            .values(player)
            .returning(PlayerEntity::as_returning())
            .get_result(&mut conn)
            .optional();
    
        match new_player {
            Ok(result) => result,
            Err(err) => panic!("Some error ocourred while trying to save card, {}", err),
        }
        
    }
    
    pub fn delete(player_id: i64) -> Result<i64, String> {
        let mut conn = db::establish_connection();
        let result = diesel::delete(players_tb.filter(id.eq(&player_id))).execute(&mut conn).optional();
    
        match result {
            Ok(Some(_)) => Ok(player_id),
            Ok(None) => Err(format!("Any player found with id {}!", player_id)),
            Err(_) => panic!("An error occured while deleting player {}", player_id),
        }
    }
    
    pub fn update(player: PlayerEntity) -> PlayerEntity {
        let mut conn = db::establish_connection();
        match player.save_changes::<PlayerEntity>(&mut conn) {
            Ok(updated_player) => updated_player,
            Err(err) => panic!("Error updating player '{}' --> {}", player.get_id(), err),
        }
        
    }

    pub fn truncate() -> Result<(), diesel::result::Error> {
        println!("TRUNCATING TABLE!!!");

        let mut conn = db::establish_connection();
        diesel::delete(players_tb)
            .execute(&mut conn)
            .map(|_| ())
    }
}


pub mod teams_dao {
    
    use crate::schema::teams_tb::dsl::teams_tb;
    use crate::schema::teams_tb as primary_teams_tb;
    use primary_teams_tb::id;
    use crate::models::{ TeamEntity, NewTeamEntity };
    use crate::db;
    use diesel::prelude::*;
    use diesel::QueryDsl;



    pub fn get_all() -> Vec<TeamEntity> {
        let mut conn = db::establish_connection();
        let results = teams_tb.load::<TeamEntity>(&mut conn)
            .expect("Ou tu é muito burro ou eu ratiei no código"); //TODO usar .optional() e criar um handler
        results
    }
    
    pub fn get_by_id(team_id: i64) -> Option<TeamEntity> {
        let mut conn = db::establish_connection();
        let result = teams_tb
            .filter(id.eq(&team_id))
            .first(&mut conn)
            .optional();

        match result {
            Ok(opt) => opt,
            Err(_) => None,
        }

        
    }
    
    pub fn save(team: &NewTeamEntity) -> Option<TeamEntity> {
        let mut conn = db::establish_connection();
        let new_team = diesel::insert_into(primary_teams_tb::table)
            .values(team)
            .returning(TeamEntity::as_returning())
            .get_result(&mut conn)
            .optional();
    
        match new_team {
            Ok(result) => result,
            Err(err) => panic!("Some error ocourred while trying to save card, {}", err),
        }
        
    }
    
    pub fn delete(team_id: i64) -> Result<i64, String> {
        let mut conn = db::establish_connection();
        let result = diesel::delete(teams_tb.filter(id.eq(&team_id))).execute(&mut conn).optional();
    
        match result {
            Ok(Some(_)) => Ok(team_id),
            Ok(None) => Err(format!("Any team found with id {}!", team_id)),
            Err(_) => panic!("An error occured while deleting team {}", team_id),
        }
    }
    
    pub fn update(team: TeamEntity) -> TeamEntity {
        let mut conn = db::establish_connection();
        match team.save_changes::<TeamEntity>(&mut conn) {
            Ok(updated_team) => updated_team,
            Err(err) => panic!("Error updating team '{}' --> {}", team.get_id(), err),
        }
        
    }

    pub fn truncate() -> Result<(), diesel::result::Error> {
        println!("TRUNCATING TABLE!!!");
        let mut conn = db::establish_connection();
        diesel::delete(teams_tb)
            .execute(&mut conn)
            .map(|_| ())
    }
}


pub mod tables_dao {
    
    use crate::schema::tables_tb::dsl::tables_tb;
    use crate::schema::tables_tb as primary_tables_tb;
    use primary_tables_tb::id;
    use crate::models::{ TableEntity, NewTableEntity };
    use crate::db;
    use diesel::prelude::*;
    use diesel::QueryDsl;
    use diesel::RunQueryDsl;



    pub fn get_all() -> Vec<TableEntity> {
        let mut conn = db::establish_connection();

        let results = tables_tb
            .load::<TableEntity>(&mut conn)
            .expect("Ou tu é muito burro ou eu ratiei no código"); //TODO usar .optional() e criar um handler
        results
    }
    
    pub fn get_by_id(table_id: i64) -> Option<TableEntity> {
        let mut conn = db::establish_connection();
        let result = tables_tb
            .filter(id.eq(&table_id))
            .first(&mut conn)
            .optional();

        match result {
            Ok(opt) => opt,
            Err(_) => None,
        }
    }
    
    pub fn save(table: &NewTableEntity) -> Option<TableEntity> {
        let mut conn = db::establish_connection();
        let new_table = diesel::insert_into(primary_tables_tb::table)
            .values(table)
            .returning(TableEntity::as_returning())
            .get_result(&mut conn)
            .optional();
    
        match new_table {
            Ok(result) => result,
            Err(err) => panic!("Some error ocourred while trying to save card, {}", err),
        }
        
    }
    
    pub fn delete(table_id: i64) -> Result<i64, String> {
        let mut conn = db::establish_connection();
        let result = diesel::delete(tables_tb.filter(id.eq(&table_id))).execute(&mut conn).optional();
    
        match result {
            Ok(Some(_)) => Ok(table_id),
            Ok(None) => Err(format!("Any table found with id {}!", table_id)),
            Err(_) => panic!("An error occured while deleting table {}", table_id),
        }
    }
    
    pub fn update(table: TableEntity) -> TableEntity {
        let mut conn = db::establish_connection();
        match table.save_changes::<TableEntity>(&mut conn) {
            Ok(updated_table) => updated_table,
            Err(err) => panic!("Error updating table '{}' --> {}", table.get_id(), err),
        }
        
    }

    pub fn truncate() -> Result<(), diesel::result::Error> {

        println!("TRUNCATING TABLE!!!");
        let mut conn = db::establish_connection();
        diesel::delete(tables_tb)
            .execute(&mut conn)
            .map(|_| ())
    }
}

pub mod cards_service {

    use crate::models::{ NewCardEntity, CardEntity };
    use super::cards_dao::{*, self};

    pub fn get_all() -> Vec<CardEntity> {
        cards_dao::get_all()
    }

    pub fn get_by_id(card_id: i64) -> CardEntity {
        let queried_card = cards_dao::get_by_id(card_id);

        match queried_card {
            Some(card) => card,
            None => panic!("Any card with ID '{}' found.", card_id)
        }
    }

    pub fn get_player_cards(player_id: i64) -> Vec<CardEntity> {
        let player_cards = cards_dao::get_player_cards(player_id);
        if player_cards.is_empty() {
            panic!("Player {} does not have any card", player_id);
        }
        player_cards
    }

    pub fn insert(new_card: NewCardEntity) -> CardEntity {
        let saved_card = cards_dao::save(&new_card);

        match saved_card {
            Some(card) => card,
            None => panic!("Card was not saved.")
        }
    }
    
}


pub mod players_service {
    use crate::models::*;
    use super::players_dao::{*, self};

    pub fn get_players_by_team_id(team_id: i64) -> Vec<PlayerEntity> {
        let player_cards = players_dao::get_players_by_team_id(team_id);
        if player_cards.is_empty() {
            panic!("Team {} does not have any player", team_id);
        }
        player_cards
    }
}


pub mod tables_service {
    use crate::models::*;

}