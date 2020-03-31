use crate::data::{Card, CardCategory, Node, UuidTable, Wave, ID};
use crate::database_schema::{cards, cards_with_pageinfo, global_uuids, waves};
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

pub fn get_wave(connection: &PgConnection, id: ID) -> QueryResult<Wave> {
  waves::table
    .filter(waves::id.eq(id))
    .first::<Wave>(connection)
}

pub fn get_card(connection: &PgConnection, id: ID) -> QueryResult<Card> {
  cards_with_pageinfo::table
    .filter(cards_with_pageinfo::id.eq(id))
    .first::<Card>(connection)
}

pub fn get_node(connection: &PgConnection, id: ID) -> QueryResult<Node> {
  let which_table = global_uuids::table
    .select(global_uuids::in_table)
    .filter(global_uuids::id.eq(id))
    .first::<UuidTable>(connection)?;

  let node = match which_table {
    UuidTable::Waves => Node::from(get_wave(connection, id)?),
    UuidTable::Cards => Node::from(get_card(connection, id)?),
  };

  Ok(node)
}

pub fn get_cards(connection: &PgConnection, pagination: Pagination) -> QueryResult<Vec<Card>> {
  // ID and Cursor are interchangable
  let subselect = |cursor| {
    // This is using the cards table instead of the view because Diesel sucks
    cards::table
      .select(cards::sort_order)
      .filter(cards::id.eq(cursor))
      .single_value()
  };

  let cards = match pagination {
    Pagination::None => cards_with_pageinfo::table.load::<Card>(connection)?,
    Pagination::Before(before_cursor) => {
      let before_subselect = subselect(before_cursor);

      cards_with_pageinfo::table
        .filter(
          cards_with_pageinfo::sort_order
            .nullable()
            .lt(before_subselect),
        )
        .order(cards_with_pageinfo::sort_order.asc())
        .load::<Card>(connection)?
    }
    Pagination::After(after_cursor) => {
      let after_subselect = subselect(after_cursor);

      cards_with_pageinfo::table
        .filter(
          cards_with_pageinfo::sort_order
            .nullable()
            .gt(after_subselect),
        )
        .order(cards_with_pageinfo::sort_order.asc())
        .load::<Card>(connection)?
    }
    Pagination::Betwixt(before_cursor, after_cursor) => {
      let before_subselect = subselect(before_cursor);
      let after_subselect = subselect(after_cursor);

      cards_with_pageinfo::table
        .filter(
          cards_with_pageinfo::sort_order
            .nullable()
            .gt(after_subselect),
        )
        .filter(
          cards_with_pageinfo::sort_order
            .nullable()
            .lt(before_subselect),
        )
        .order(cards_with_pageinfo::sort_order.asc())
        .load::<Card>(connection)?
    }
    Pagination::First(limit) => cards_with_pageinfo::table
      .order(cards_with_pageinfo::sort_order.asc())
      .limit(limit)
      .load::<Card>(connection)?,
    Pagination::FirstAfter(limit, after_cursor) => {
      let after_subselect = subselect(after_cursor);

      cards_with_pageinfo::table
        .filter(
          cards_with_pageinfo::sort_order
            .nullable()
            .gt(after_subselect),
        )
        .order(cards_with_pageinfo::sort_order.asc())
        .limit(limit)
        .load::<Card>(connection)?
    }
    Pagination::FirstBefore(limit, before_cursor) => {
      let before_subselect = subselect(before_cursor);

      cards_with_pageinfo::table
        .filter(
          cards_with_pageinfo::sort_order
            .nullable()
            .lt(before_subselect),
        )
        .order(cards_with_pageinfo::sort_order.asc())
        .limit(limit)
        .load::<Card>(connection)?
    }
    Pagination::FirstBetwixt(limit, before_cursor, after_cursor) => {
      let before_subselect = subselect(before_cursor);
      let after_subselect = subselect(after_cursor);

      cards_with_pageinfo::table
        .filter(
          cards_with_pageinfo::sort_order
            .nullable()
            .gt(after_subselect),
        )
        .filter(
          cards_with_pageinfo::sort_order
            .nullable()
            .lt(before_subselect),
        )
        .order(cards_with_pageinfo::sort_order.asc())
        .limit(limit)
        .load::<Card>(connection)?
    }
    Pagination::Last(limit) => cards_with_pageinfo::table
      .order(cards_with_pageinfo::sort_order.desc())
      .limit(limit)
      .load::<Card>(connection)?
      .iter()
      .cloned()
      .rev()
      .collect::<Vec<Card>>(),
    Pagination::LastAfter(limit, after_cursor) => {
      let after_subselect = subselect(after_cursor);

      cards_with_pageinfo::table
        .filter(
          cards_with_pageinfo::sort_order
            .nullable()
            .gt(after_subselect),
        )
        .order(cards_with_pageinfo::sort_order.desc())
        .limit(limit)
        .load::<Card>(connection)?
        .iter()
        .cloned()
        .rev()
        .collect::<Vec<Card>>()
    }
    Pagination::LastBefore(limit, before_cursor) => {
      let before_subselect = subselect(before_cursor);

      cards_with_pageinfo::table
        .filter(
          cards_with_pageinfo::sort_order
            .nullable()
            .lt(before_subselect),
        )
        .order(cards_with_pageinfo::sort_order.desc())
        .limit(limit)
        .load::<Card>(connection)?
        .iter()
        .cloned()
        .rev()
        .collect::<Vec<Card>>()
    }
    Pagination::LastBetwixt(limit, before_cursor, after_cursor) => {
      let before_subselect = subselect(before_cursor);
      let after_subselect = subselect(after_cursor);

      cards_with_pageinfo::table
        .filter(
          cards_with_pageinfo::sort_order
            .nullable()
            .gt(after_subselect),
        )
        .filter(
          cards_with_pageinfo::sort_order
            .nullable()
            .lt(before_subselect),
        )
        .order(cards_with_pageinfo::sort_order.desc())
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
