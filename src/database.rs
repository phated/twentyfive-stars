pub mod schema;
pub mod types;

pub use types::{CardCategory, CharacterTrait, ModeType};

use crate::models::{BattleCard, Card, CharacterCard, Wave};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use schema::{battle_cards, cards, character_modes, waves};
use std::env;

pub fn establish_connection() -> PgConnection {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        PgConnection::establish(&database_url)
                .expect(&format!("Error connecting to {}", database_url))
}

pub fn get_cards(connection: &PgConnection) -> QueryResult<Vec<Card>> {
        let battles = cards::table
                .filter(cards::category.eq(CardCategory::Battle))
                .inner_join(battle_cards::table)
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
                .map(Card::from);

        let characters = cards::table
                .filter(cards::category.eq(CardCategory::Character))
                .load::<types::Card>(connection)?
                .into_iter()
                .map(|card| {
                        let wave = waves::table
                                .filter(waves::id.eq(card.wave_id))
                                .first::<Wave>(connection)
                                .expect("Where's the wave?");
                        let modes = types::CharacterMode::belonging_to(&card)
                                .load::<types::CharacterMode>(connection)
                                .expect("");
                        let character = CharacterCard::new(card, wave, modes);
                        Card::from(character)
                });

        let cards = battles.chain(characters).collect();

        Ok(cards)
}
