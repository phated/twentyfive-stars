use crate::data::{Card, CardCategory};
use crate::database_schema::{cards, cards_with_pageinfo};
use crate::pagination::Pagination;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
  dotenv().ok();

  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn get_cards(connection: &PgConnection, pagination: Pagination) -> QueryResult<Vec<Card>> {
  let subselect = |external_id| {
    cards::table
      .select(cards::id)
      .filter(cards::external_id.eq(external_id))
      .single_value()
  };

  let cards = match pagination {
    Pagination::None => cards_with_pageinfo::table.load::<Card>(connection)?,
    Pagination::Before(before_external_id) => {
      let before_subselect = subselect(before_external_id);

      cards_with_pageinfo::table
        .filter(cards_with_pageinfo::id.nullable().lt(before_subselect))
        .order(cards_with_pageinfo::id.asc())
        .load::<Card>(connection)?
    }
    Pagination::After(after_external_id) => {
      let after_subselect = subselect(after_external_id);

      cards_with_pageinfo::table
        .filter(cards_with_pageinfo::id.nullable().gt(after_subselect))
        .order(cards_with_pageinfo::id.asc())
        .load::<Card>(connection)?
    }
    Pagination::Betwixt(before_external_id, after_external_id) => {
      let before_subselect = subselect(before_external_id);
      let after_subselect = subselect(after_external_id);

      cards_with_pageinfo::table
        .filter(cards_with_pageinfo::id.nullable().gt(after_subselect))
        .filter(cards_with_pageinfo::id.nullable().lt(before_subselect))
        .order(cards_with_pageinfo::id.asc())
        .load::<Card>(connection)?
    }
    Pagination::First(limit) => cards_with_pageinfo::table
      .order(cards_with_pageinfo::id.asc())
      .limit(limit)
      .load::<Card>(connection)?,
    Pagination::FirstAfter(limit, after_external_id) => {
      let after_subselect = subselect(after_external_id);

      cards_with_pageinfo::table
        .filter(cards_with_pageinfo::id.nullable().gt(after_subselect))
        .order(cards_with_pageinfo::id.asc())
        .limit(limit)
        .load::<Card>(connection)?
    }
    Pagination::FirstBefore(limit, before_external_id) => {
      let before_subselect = subselect(before_external_id);

      cards_with_pageinfo::table
        .filter(cards_with_pageinfo::id.nullable().lt(before_subselect))
        .order(cards_with_pageinfo::id.asc())
        .limit(limit)
        .load::<Card>(connection)?
    }
    Pagination::FirstBetwixt(limit, before_external_id, after_external_id) => {
      let before_subselect = subselect(before_external_id);
      let after_subselect = subselect(after_external_id);

      cards_with_pageinfo::table
        .filter(cards_with_pageinfo::id.nullable().gt(after_subselect))
        .filter(cards_with_pageinfo::id.nullable().lt(before_subselect))
        .order(cards_with_pageinfo::id.asc())
        .limit(limit)
        .load::<Card>(connection)?
    }
    Pagination::Last(limit) => cards_with_pageinfo::table
      .order(cards_with_pageinfo::id.desc())
      .limit(limit)
      .load::<Card>(connection)?
      .iter()
      .cloned()
      .rev()
      .collect::<Vec<Card>>(),
    Pagination::LastAfter(limit, after_external_id) => {
      let after_subselect = subselect(after_external_id);

      cards_with_pageinfo::table
        .filter(cards_with_pageinfo::id.nullable().gt(after_subselect))
        .order(cards_with_pageinfo::id.desc())
        .limit(limit)
        .load::<Card>(connection)?
        .iter()
        .cloned()
        .rev()
        .collect::<Vec<Card>>()
    }
    Pagination::LastBefore(limit, before_external_id) => {
      let before_subselect = subselect(before_external_id);

      cards_with_pageinfo::table
        .filter(cards_with_pageinfo::id.nullable().lt(before_subselect))
        .order(cards_with_pageinfo::id.desc())
        .limit(limit)
        .load::<Card>(connection)?
        .iter()
        .cloned()
        .rev()
        .collect::<Vec<Card>>()
    }
    Pagination::LastBetwixt(limit, before_external_id, after_external_id) => {
      let before_subselect = subselect(before_external_id);
      let after_subselect = subselect(after_external_id);

      cards_with_pageinfo::table
        .filter(cards_with_pageinfo::id.nullable().gt(after_subselect))
        .filter(cards_with_pageinfo::id.nullable().lt(before_subselect))
        .order(cards_with_pageinfo::id.desc())
        .limit(limit)
        .load::<Card>(connection)?
        .iter()
        .cloned()
        .rev()
        .collect::<Vec<Card>>()
    }
    // TODO: How can I error?
    Pagination::Invalid => Vec::new(),
  };

  Ok(cards)
}

pub fn get_character_cards(connection: &PgConnection) -> QueryResult<Vec<Card>> {
  let cards = cards_with_pageinfo::table
    .filter(cards_with_pageinfo::category.eq(CardCategory::Character))
    .load::<Card>(connection)?;

  Ok(cards)
}

pub fn get_battle_cards(connection: &PgConnection) -> QueryResult<Vec<Card>> {
  let cards = cards_with_pageinfo::table
    .filter(cards_with_pageinfo::category.eq(CardCategory::Battle))
    .load::<Card>(connection)?;

  Ok(cards)
}

pub fn get_stratagem_cards(connection: &PgConnection) -> QueryResult<Vec<Card>> {
  let cards = cards_with_pageinfo::table
    .filter(cards_with_pageinfo::category.eq(CardCategory::Stratagem))
    .load::<Card>(connection)?;

  Ok(cards)
}
