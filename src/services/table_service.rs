use crate::{models::{ table::TableEntity, team::TeamEntity, card::CardEntity, player::PlayerEntity, projections::{TableProjection, CardProjection, TeamProjection, PlayerProjection}  }, utils::CardsShuffler};
use crate::dao::{ table_dao, team_dao, player_dao, card_dao };
use log::info;
use sqlx::{ PgPool, Error};
use crate::utils::is_manilha;

pub async fn setup_table(table_id: i64, player_id: i64, pool: &PgPool) -> Result<TableProjection, Error> {
    let table = table_dao::get_by_id(table_id, pool).await?;
    let teams: Vec<TeamEntity> = team_dao::get_table_teams(table_id, pool).await?;

    if teams.len() == 2 {
        let mut players: Vec<PlayerEntity> = vec![];
        for t in &teams {
            players.append(&mut player_dao::get_team_players(t.id, pool).await?);
        }

        match players.iter().find(|p| p.id == player_id) {
            Some(_) => panic!("TU JA TA NO BAGULHOOOOOOOOOOOOOO, SAE DAE BURRO"),
            None => (),
        }

        let player_team_id = arbitrarily_set_team(&mut players, &teams);

        players.push(add_new_player_to_team(player_team_id, player_id, pool).await?);
        
        if players.len() >= 4 {
            let mut card_shuffler = CardsShuffler::new();
            let mut shuffled_cards = card_shuffler.get_shuffled_cards();
            let maquina = card_shuffler.get_card();
            let new_maquina = card_dao::save(maquina, pool).await?;

            table_dao::set_table_maquina(table_id, new_maquina.id, pool).await?;

            let mut player_cards: Vec<CardEntity> = vec![];
            for index in 0..4{
                for card in &mut shuffled_cards[index] {
                    card.is_manilha = is_manilha(&maquina, &card);
                    card.player_entity_id = Some(players[index].id);
                }
                player_cards.append(&mut card_dao::save_cards(shuffled_cards[index].clone(), pool).await?);
            }

            return Ok(get_table_projection(table, Some(CardProjection::new(new_maquina.id, new_maquina.suit, new_maquina.card_value, new_maquina.is_manilha)), teams, players, player_cards, true)) 
        } else {
            info!("{:#?}", players);
            return Ok(get_table_projection(table, None, teams, players, vec![], false)) 
        }
    } else {
        let new_teams = team_dao::create_two_empty_teams(table_id, pool).await?;
        let player = player_dao::add_player_to_team(player_id, new_teams[0].id, pool).await?;
        return Ok(get_table_projection(table, None, new_teams, vec![player], vec![], false))
    }
}

///Return a team for the player
fn arbitrarily_set_team(players: &mut Vec<PlayerEntity>, teams: &Vec<TeamEntity>) -> Option<i64> {
    if teams.len() > 2 || teams.len() <= 0 {
        return None
    }
    let team_one_count = players.iter().filter(|player: &&PlayerEntity| player.team_entity_id == Some(teams[0].id)).count() as i64;
    let team_two_count = players.iter().filter(|player: &&PlayerEntity| player.team_entity_id == Some(teams[1].id)).count() as i64;

    if team_one_count < 2 {
        return Some(teams[0].id)
    } else if team_two_count < 2 {
        return Some(teams[1].id)
    } else {
        None
    }

}

async fn add_new_player_to_team(player_team_id: Option<i64>, player_id: i64, conn: &PgPool) -> Result<PlayerEntity, Error>{
    if let Some(team_id) = player_team_id {
        let player = player_dao::add_player_to_team(player_id, team_id, conn).await?;
        Ok(player)
    } else {
        panic!("");
    }
}

fn get_table_projection(t: TableEntity, m: Option<CardProjection>, teams: Vec<TeamEntity>, players: Vec<PlayerEntity>, cards: Vec<CardEntity>, start: bool) -> TableProjection {
    TableProjection::new(
        t.id,
        t.last_played,
        m,
        teams.into_iter().map(|t| TeamProjection::new(
            t.id,
            t.score,
            players.iter().filter_map(|p| {
                if p.team_entity_id == Some(t.id) {
                    return Some(PlayerProjection::new(
                        p.id,
                        p.name.clone(),
                        cards.iter().filter_map(|c| {
                            if c.player_entity_id == Some(p.id) {
                                return Some(CardProjection::new(
                                    c.id,
                                    c.suit,
                                    c.card_value,
                                    c.is_manilha,
                                ))
                            }
                            None
                        }).collect(),
                    ))
                }
                None
            }).collect(),
        )).collect(),
        start
    )
}