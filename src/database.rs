pub mod models;
pub mod schema;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use crate::database::models::{Card};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn get_cards(connection: &PgConnection) -> Vec<Card> {
    schema::cards::table
        // .select()
        .load::<Card>(connection)
        .expect("Error loading posts")
}
