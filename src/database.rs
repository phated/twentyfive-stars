pub mod schema;
pub mod types;

use crate::models::{BattleCard, Card};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use schema::{battle_cards, cards};
use std::env;

pub fn establish_connection() -> PgConnection {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        PgConnection::establish(&database_url)
                .expect(&format!("Error connecting to {}", database_url))
}

pub fn get_cards(connection: &PgConnection) -> QueryResult<Vec<Card>> {
        let cards = battle_cards::table
                .inner_join(cards::table)
                .select((
                        battle_cards::id,
                        battle_cards::card_id,
                        cards::tcg_id,
                        cards::rarity,
                        cards::number,
                        cards::category,
                        cards::wave_id,
                        // Specific to BattleCard
                        battle_cards::title,
                        battle_cards::stars,
                        battle_cards::icons,
                        battle_cards::type_,
                        battle_cards::attack_modifier,
                        battle_cards::defense_modifier,
                ))
                .load::<BattleCard>(connection)?
                .into_iter()
                .map(Card::from)
                .collect();

        Ok(cards)
}
