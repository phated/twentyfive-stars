use crate::data::{Card, CardCategory, Wave, ID};
use crate::database_schema::{cards, cards_with_pageinfo, waves};
use crate::pagination::Pagination;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

pub type ConnPool = Pool<ConnectionManager<PgConnection>>;
pub type Error = Box<dyn std::error::Error + Send + Sync>;

pub struct Database {
  pool: ConnPool,
}

impl Database {
  pub fn new(database_url: &str) -> Result<Database, Error> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder().build(manager)?;
    let db = Database { pool };
    Ok(db)
  }
}

impl Database {
  pub fn get_wave(&self, id: ID) -> Result<Wave, Error> {
    let conn = self.pool.get()?;
    let wave = waves::table.filter(waves::id.eq(id)).first::<Wave>(&conn)?;
    Ok(wave)
  }

  pub fn get_card(&self, id: ID) -> Result<Card, Error> {
    let conn = self.pool.get()?;
    let card = cards_with_pageinfo::table
      .filter(cards_with_pageinfo::id.eq(id))
      .first::<Card>(&conn)?;
    Ok(card)
  }

  // pub fn get_node(connection: &PgConnection, id: ID) -> QueryResult<Node> {
  //   let which_table = global_uuids::table
  //     .select(global_uuids::in_table)
  //     .filter(global_uuids::id.eq(id))
  //     .first::<UuidTable>(&conn)?;

  //   let node = match which_table {
  //     UuidTable::Waves => Node::from(get_wave(connection, id)?),
  //     UuidTable::Cards => Node::from(get_card(connection, id)?),
  //   };

  //   Ok(node)
  // }

  pub fn get_character_cards(&self) -> Result<Vec<Card>, Error> {
    let conn = self.pool.get()?;

    let cards = cards_with_pageinfo::table
      .filter(cards_with_pageinfo::category.eq(CardCategory::Character))
      .load::<Card>(&conn)?;

    Ok(cards)
  }

  pub fn get_battle_cards(&self) -> Result<Vec<Card>, Error> {
    let conn = self.pool.get()?;

    let cards = cards_with_pageinfo::table
      .filter(cards_with_pageinfo::category.eq(CardCategory::Battle))
      .load::<Card>(&conn)?;

    Ok(cards)
  }

  pub fn get_stratagem_cards(&self) -> Result<Vec<Card>, Error> {
    let conn = self.pool.get()?;

    let cards = cards_with_pageinfo::table
      .filter(cards_with_pageinfo::category.eq(CardCategory::Stratagem))
      .load::<Card>(&conn)?;

    Ok(cards)
  }

  pub fn get_cards(&self, pagination: Pagination) -> Result<Vec<Card>, Error> {
    let conn = self.pool.get()?;

    // ID and Cursor are interchangable
    let subselect = |cursor| {
      // This is using the cards table instead of the view because Diesel sucks
      cards::table
        .select(cards::sort_order)
        .filter(cards::id.eq(cursor))
        .single_value()
    };

    let cards = match pagination {
      Pagination::None => cards_with_pageinfo::table.load::<Card>(&conn)?,
      Pagination::Before(before_cursor) => {
        let before_subselect = subselect(before_cursor);

        cards_with_pageinfo::table
          .filter(
            cards_with_pageinfo::sort_order
              .nullable()
              .lt(before_subselect),
          )
          .order(cards_with_pageinfo::sort_order.asc())
          .load::<Card>(&conn)?
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
          .load::<Card>(&conn)?
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
          .load::<Card>(&conn)?
      }
      Pagination::First(limit) => cards_with_pageinfo::table
        .order(cards_with_pageinfo::sort_order.asc())
        .limit(limit)
        .load::<Card>(&conn)?,
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
          .load::<Card>(&conn)?
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
          .load::<Card>(&conn)?
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
          .load::<Card>(&conn)?
      }
      Pagination::Last(limit) => cards_with_pageinfo::table
        .order(cards_with_pageinfo::sort_order.desc())
        .limit(limit)
        .load::<Card>(&conn)?
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
          .load::<Card>(&conn)?
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
          .load::<Card>(&conn)?
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
          .load::<Card>(&conn)?
          .iter()
          .cloned()
          .rev()
          .collect::<Vec<Card>>()
      }
      // TODO: How can I error?
      Pagination::Invalid => Vec::new(),
    };

    Ok(cards.into_iter().map(|card| card.into()).collect())
  }
}
