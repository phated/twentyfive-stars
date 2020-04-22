use crate::data::{Card, Wave, ID};
use crate::database_schema::{cards, cards_with_pageinfo, waves};
use crate::pagination::Pagination;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use std::env;
use std::error::Error;
use tokio_diesel::*;

pub type ConnPool = Pool<ConnectionManager<PgConnection>>;

pub fn pool() -> Result<ConnPool, Box<dyn Error>> {
  dotenv().ok();

  let database_url = env::var("DATABASE_URL")?;
  let manager = ConnectionManager::<PgConnection>::new(database_url);
  let pool = Pool::builder().build(manager)?;

  Ok(pool)
}

pub async fn get_wave(pool: &ConnPool, id: ID) -> AsyncResult<Wave> {
  waves::table
    .filter(waves::id.eq(id))
    .first_async::<Wave>(pool)
    .await
}

// pub fn get_card(connection: &PgConnection, id: ID) -> QueryResult<Card> {
//   cards_with_pageinfo::table
//     .filter(cards_with_pageinfo::id.eq(id))
//     .first::<Card>(connection)
// }

// pub fn get_node(connection: &PgConnection, id: ID) -> QueryResult<Node> {
//   let which_table = global_uuids::table
//     .select(global_uuids::in_table)
//     .filter(global_uuids::id.eq(id))
//     .first::<UuidTable>(pool).await?;

//   let node = match which_table {
//     UuidTable::Waves => Node::from(get_wave(connection, id)?),
//     UuidTable::Cards => Node::from(get_card(connection, id)?),
//   };

//   Ok(node)
// }

pub async fn get_cards(pool: &ConnPool, pagination: Pagination) -> AsyncResult<Vec<Card>> {
  // ID and Cursor are interchangable
  let subselect = |cursor| {
    // This is using the cards table instead of the view because Diesel sucks
    cards::table
      .select(cards::sort_order)
      .filter(cards::id.eq(cursor))
      .single_value()
  };

  let cards = match pagination {
    Pagination::None => cards_with_pageinfo::table.load_async::<Card>(pool).await?,
    Pagination::Before(before_cursor) => {
      let before_subselect = subselect(before_cursor);

      cards_with_pageinfo::table
        .filter(
          cards_with_pageinfo::sort_order
            .nullable()
            .lt(before_subselect),
        )
        .order(cards_with_pageinfo::sort_order.asc())
        .load_async::<Card>(pool)
        .await?
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
        .load_async::<Card>(pool)
        .await?
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
        .load_async::<Card>(pool)
        .await?
    }
    Pagination::First(limit) => {
      cards_with_pageinfo::table
        .order(cards_with_pageinfo::sort_order.asc())
        .limit(limit)
        .load_async::<Card>(pool)
        .await?
    }
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
        .load_async::<Card>(pool)
        .await?
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
        .load_async::<Card>(pool)
        .await?
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
        .load_async::<Card>(pool)
        .await?
    }
    Pagination::Last(limit) => cards_with_pageinfo::table
      .order(cards_with_pageinfo::sort_order.desc())
      .limit(limit)
      .load_async::<Card>(pool)
      .await?
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
        .load_async::<Card>(pool)
        .await?
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
        .load_async::<Card>(pool)
        .await?
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
        .load_async::<Card>(pool)
        .await?
        .iter()
        .cloned()
        .rev()
        .collect::<Vec<Card>>()
    }
    // TODO: How can I error?
    Pagination::Invalid => Vec::new(),
  };

  Ok(cards.into_iter().map(|card| card.into()).collect())
  // Ok(cards)
}

// pub fn get_character_cards(connection: &PgConnection) -> QueryResult<Vec<Card>> {
//   let cards = cards_with_pageinfo::table
//     .filter(cards_with_pageinfo::category.eq(CardCategory::Character))
//     .load_async::<Card>(pool).await?;

//   Ok(cards)
// }

// pub fn get_battle_cards(connection: &PgConnection) -> QueryResult<Vec<Card>> {
//   let cards = cards_with_pageinfo::table
//     .filter(cards_with_pageinfo::category.eq(CardCategory::Battle))
//     .load_async::<Card>(pool).await?;

//   Ok(cards)
// }

// pub fn get_stratagem_cards(connection: &PgConnection) -> QueryResult<Vec<Card>> {
//   let cards = cards_with_pageinfo::table
//     .filter(cards_with_pageinfo::category.eq(CardCategory::Stratagem))
//     .load_async::<Card>(pool).await?;

//   Ok(cards)
// }
