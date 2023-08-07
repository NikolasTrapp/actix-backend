use crate::models::card::NewCardEntity;
use rand::Rng;
use sqlx::postgres::PgPool;


pub struct AppState {
    pub db: PgPool,
}

pub struct CardsShuffler {
    cards: Vec<NewCardEntity>
}

impl CardsShuffler {

    pub fn new() -> Self {
        Self { cards: NewCardEntity::get_cards_list() }
    }

    pub fn get_shuffled_cards(&mut self) -> Vec<Vec<NewCardEntity>> {
        let mut matrix: Vec<Vec<NewCardEntity>> = vec![vec![]; 4];
        for i in 0..12 {
            matrix[i % 4].push(self.get_card());
        }
        matrix
    }

    pub fn get_card(&mut self) -> NewCardEntity {
        let r_number = generate_random_number(self.cards.len()-1);
        self.cards.remove(r_number)
    }
}

pub fn generate_random_number(x: usize) -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..=x)
}


pub fn is_manilha<T>(maquina: &T, card: &T) -> bool
where
    T: crate::models::card::CardFunctions,
{
    match maquina.card_value() {
        3 => card.card_value() == 4,
        12 => card.card_value() == 1,
        7 => card.card_value() == 10,
        all_other_values => all_other_values == card.card_value() - 1
    }
}