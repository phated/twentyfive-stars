use crate::data::{Card, CardCategory};
use crate::database_schema::cards;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        PgConnection::establish(&database_url)
                .expect(&format!("Error connecting to {}", database_url))
}

pub fn get_cards(connection: &PgConnection) -> QueryResult<Vec<Card>> {
        let cards = cards::table.load::<Card>(connection)?;

        Ok(cards)
}

pub fn get_character_cards(connection: &PgConnection) -> QueryResult<Vec<Card>> {
        let cards = cards::table
                .filter(cards::category.eq(CardCategory::Character))
                .load::<Card>(connection)?;

        Ok(cards)
}

pub fn get_battle_cards(connection: &PgConnection) -> QueryResult<Vec<Card>> {
        let cards = cards::table
                .filter(cards::category.eq(CardCategory::Battle))
                .load::<Card>(connection)?;

        Ok(cards)
}
