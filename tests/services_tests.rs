#[cfg(test)]
mod table_dao_tests {
    use actix_backend::models::{TableEntity, NewTableEntity};
    use actix_backend::services::tables_dao::{ get_all, save, get_by_id };
    

    #[test]
    fn test_save_table_entity() {
        let mock_table = NewTableEntity {
            id: None,
            last_played: Some(1),
        };

        let saved_table: Option<TableEntity> = save(&mock_table);
        assert!(saved_table.is_some());
    }

    #[test]
    fn test_get_table_by_id() {
        let mock_table = NewTableEntity {
            id: None,
            last_played: Some(1),
        };

        let saved_table: TableEntity = save(&mock_table).unwrap();
        let requested_table = if let Some(x) = get_by_id(saved_table.id) {
            x
        } else {
            panic!("Not table found with id {}", saved_table.id);
        };
        assert!(saved_table.eq(&requested_table));
        
    }

    pub fn return_mock_table() -> NewTableEntity {
        NewTableEntity { 
            id: Some(999),
            last_played: None,
        } 
    }

    pub fn return_mock_table_with_values(id: Option<i64>, last_played: Option<i64>) -> NewTableEntity {
        NewTableEntity { id, last_played }
    }

}

#[cfg(test)]
mod team_dao_tests {
    use core::panic;

    use actix_backend::models::{TeamEntity, NewTeamEntity};
    use actix_backend::services::teams_dao::{ get_all, save, get_by_id };
    use actix_backend::services::tables_dao::{ save as save_table, get_all as get_all_tables };
    use crate::table_dao_tests::{return_mock_table};

    #[test]
    fn test_save_team_entity() {
        let mock_team = NewTeamEntity {
            id: None,
            score: 0,
            table_entity_id: 1,
        };

        let saved_team: Option<TeamEntity> = save(&mock_team);
        assert!(saved_team.is_some());

    }

    fn test_get_team_by_id() {
        let mock_tables = get_all_tables();
        let mocked_table_id;
        if mock_tables.is_empty() {
            mocked_table_id = save_table(&return_mock_table()).unwrap().id;
        } else {
            mocked_table_id = mock_tables[0].id;
        }

        let mock_team = NewTeamEntity {
            id: None,
            score: 0,
            table_entity_id: mocked_table_id,
        };

        let saved_team: TeamEntity = save(&mock_team).unwrap();
        let requested_team = if let Some(x) = get_by_id(saved_team.id) {
            x
        } else {
            panic!("Not table found with id {}", saved_team.id);
        };
        assert!(saved_team.eq(&requested_team));
        
    }

    pub fn return_mock_team() -> NewTeamEntity {
        NewTeamEntity { 
            id: Some(999),
            score: 0,
            table_entity_id: 1,
        } 
    }

    pub fn return_mock_team_with_values(id: Option<i64>, score: i16, table_entity_id: i64) -> NewTeamEntity {
        NewTeamEntity { id, score, table_entity_id }
    }

}

#[cfg(test)]
pub mod player_dao_tests {
    use actix_backend::models::{PlayerEntity, NewPlayerEntity};
    use actix_backend::services::players_dao::{ get_all, save, get_by_id };
    use actix_backend::services::teams_dao::{ save as save_team, get_all as get_all_teams };
    use crate::team_dao_tests::{return_mock_team};


    #[test]
    fn test_save_player_entity() {
        let mock_player = NewPlayerEntity {
            id: None,
            name: String::from("repolho"),
            victories: 0,
            team_entity_id: 1,
        };

        let saved_player: Option<PlayerEntity> = save(&mock_player);
        assert!(saved_player.is_some());
    }

    fn test_get_player_by_id() {
        let mock_teams = get_all_teams();
        let mocked_team_id;
        if mock_teams.is_empty() {
            mocked_team_id = save_team(&return_mock_team()).unwrap().id;
        } else {
            mocked_team_id = mock_teams[0].id;
        }

        let mock_player = NewPlayerEntity {
            id: None,
            victories: 0,
            name: String::from("Bananilson"),
            team_entity_id: mocked_team_id,
        };

        let saved_player: PlayerEntity = save(&mock_player).unwrap();
        let requested_player = if let Some(x) = get_by_id(saved_player.id) {
            x
        } else {
            panic!("Not table found with id {}", saved_player.id);
        };
        assert!(saved_player.eq(&requested_player));
    
    }

    pub fn return_mock_player() -> NewPlayerEntity {
        NewPlayerEntity { 
            id: None,
            name: String::from("Repolho"),
            victories: 0,
            team_entity_id: 1,
        } 
    }

    pub fn return_mock_player_with_values(id: Option<i64>, name: String, victories: i64, team_entity_id: i64) -> NewPlayerEntity {
        NewPlayerEntity { id, name, victories, team_entity_id, }
    }

}

#[cfg(test)]
mod cards_dao_tests {
    use actix_backend::models::{CardEntity, NewCardEntity, Suit};
    use actix_backend::schema::cards_tb::card_value;
    use actix_backend::services::cards_dao::{ get_all, save, get_by_id };
    use actix_backend::services::players_dao::{ get_all as get_all_players, save as save_player };
    use actix_backend::services::cards_service::{ get_player_cards };
    use crate::player_dao_tests::{ return_mock_player };


    #[test]
    fn test_save_card_entity() {
        let mock_player = &return_mock_player();
        save_player(&mock_player);
        let mock_card = NewCardEntity {
            id: None,
            suit: Suit::Spades,
            card_value: 1,
            is_manilha: false,
            player_entity_id: 1,
        };

        let saved_card: Option<CardEntity> = save(&mock_card);
        assert!(saved_card.is_some());
    }   

    #[test]
    fn test_get_card_by_id() {
        let mock_players = get_all_players();
        let mocked_player_id;
        if mock_players.is_empty() {
            mocked_player_id = save_player(&return_mock_player()).unwrap().id;
        } else {
            mocked_player_id = mock_players[0].id;
        }

        let mock_card = NewCardEntity {
            id: None,
            card_value: 1,
            suit: Suit::Clubs,
            is_manilha: false,
            player_entity_id: mocked_player_id,
        };

        let saved_card: CardEntity = save(&mock_card).unwrap();
        let requested_card = if let Some(x) = get_by_id(saved_card.id) {
            x
        } else {
            panic!("Not table found with id {}", saved_card.id);
        };
        assert!(saved_card.eq(&requested_card));
    
    }

    #[test]
    fn test_get_cards_by_player_id() {
        let mock_players = get_all_players();
        let mocked_player_id;
        if mock_players.is_empty() {
            mocked_player_id = save_player(&return_mock_player()).unwrap().id;
        } else {
            mocked_player_id = mock_players[0].id;
        }
        
        let cards = get_player_cards(mocked_player_id);
        for card in cards.clone() {
            println!("Cards with id: {} found.", card.id);
        }
        assert!(!cards.is_empty());
    }

}


#[cfg(test)]
mod truncate_all_test {
    use actix_backend::services::{tables_dao, teams_dao, players_dao, cards_dao };

    #[test]
    fn truncate_all_tables() {
        let _ = tables_dao::truncate();
        let _ = teams_dao::truncate();
        let _ = players_dao::truncate();
        let _ = cards_dao::truncate();

        assert!(tables_dao::get_all().is_empty());
        assert!(teams_dao::get_all().is_empty());
        assert!(players_dao::get_all().is_empty());
        assert!(cards_dao::get_all().is_empty());

    }


}