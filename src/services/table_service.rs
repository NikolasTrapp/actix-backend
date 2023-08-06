use crate::models::{ table::{ TableEntity, NewTableEntity }, team::TeamEntity, card::CardEntity, player::PlayerEntity  };
use crate::dao::{ table_dao, team_dao, player_dao, card_dao };
use sqlx::{ PgPool, Error};

pub async fn setup_table(table_id: i64, player_id: i64, pool: &PgPool) -> Result<PlayerEntity, Error> {
    let teams: Vec<TeamEntity> = team_dao::get_table_teams(table_id, pool).await?;

    if teams.len() == 2 {
        let mut players: Vec<PlayerEntity> = vec![];
        for t in &teams {
            players.append(&mut player_dao::get_team_players(t.id, pool).await?)
        }
        let player_team_id = arbitrarily_set_team(&mut players, &teams);

        //AQUI TEM QUE VER SE OS PLAYERS TÃO == 3 PRA INICIAR A PARTIDA PQ O 4° FOI INSERIDO
        if let Some(p_id) = player_team_id {
            return Ok(player_dao::add_player_to_team(player_id, p_id, pool).await?)
        } else {
            panic!("");
        }
    } else {
        let new_teams = team_dao::create_two_empty_teams(table_id, pool).await?;
        return Ok(player_dao::add_player_to_team(player_id, new_teams[0].id, pool).await?)
    }
}

///Return a team for the player
fn arbitrarily_set_team(players: &mut Vec<PlayerEntity>, teams: &Vec<TeamEntity>) -> Option<i64> {
    if teams.len() > 2 || teams.len() <= 0 {
        return None
    }
    let team_one_count = players.iter().filter(|player: &&PlayerEntity| player.team_entity_id == teams[0].id).count() as i64;
    let team_two_count = players.iter().filter(|player: &&PlayerEntity| player.team_entity_id == teams[1].id).count() as i64;

    if team_one_count < 2 {
        return Some(teams[0].id)
    } else if team_two_count < 2 {
        return Some(teams[1].id)
    } else {
        None
    }

}