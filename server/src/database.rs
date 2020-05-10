use crate::data::{Card, CardCategory, Wave, ID};
use crate::database_schema::{cards, cards_with_pageinfo, waves};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use sqlx::postgres::PgPool;
use uuid::Uuid;

// TODO: This shouldn't be in here
use async_graphql::QueryOperation;

pub type Error = Box<dyn std::error::Error + Send + Sync>;

pub struct Database {
  pool: PgPool,
}

impl Database {
  pub async fn new(database_url: &str) -> Result<Database, Error> {
    let pool = PgPool::builder()
      .max_size(5) // maximum number of connections in the pool
      .build(database_url)
      .await?;
    let db = Database { pool };
    Ok(db)
  }
}

impl Database {
  pub async fn get_wave(&self, id: ID) -> Result<Wave, Error> {
    let wave = sqlx::query_as_unchecked!(Wave, "SELECT id, tcg_id, name, released FROM waves")
      .fetch_one(&self.pool)
      .await?;
    Ok(wave)
  }

  // pub fn get_card(&self, id: ID) -> Result<Card, Error> {
  //   let conn = self.pool.get()?;
  //   let card = cards_with_pageinfo::table
  //     .filter(cards_with_pageinfo::id.eq(id))
  //     .first::<Card>(&conn)?;
  //   Ok(card)
  // }

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

  // pub fn get_character_cards(&self) -> Result<Vec<Card>, Error> {
  //   let conn = self.pool.get()?;

  //   let cards = cards_with_pageinfo::table
  //     .filter(cards_with_pageinfo::category.eq(CardCategory::Character))
  //     .load::<Card>(&conn)?;

  //   Ok(cards)
  // }

  // pub fn get_battle_cards(&self) -> Result<Vec<Card>, Error> {
  //   let conn = self.pool.get()?;

  //   let cards = cards_with_pageinfo::table
  //     .filter(cards_with_pageinfo::category.eq(CardCategory::Battle))
  //     .load::<Card>(&conn)?;

  //   Ok(cards)
  // }

  // pub fn get_stratagem_cards(&self) -> Result<Vec<Card>, Error> {
  //   let conn = self.pool.get()?;

  //   let cards = cards_with_pageinfo::table
  //     .filter(cards_with_pageinfo::category.eq(CardCategory::Stratagem))
  //     .load::<Card>(&conn)?;

  //   Ok(cards)
  // }

  pub async fn get_cards(&self, pagination: QueryOperation) -> Result<Vec<Card>, Error> {
    // let conn = self.pool.get()?;

    // ID and Cursor are interchangable
    // let subselect = |cursor| {
    //   // This is using the cards table instead of the view because Diesel sucks
    //   cards::table
    //     .select(cards::sort_order)
    //     .filter(cards::id.eq(cursor))
    //     .single_value()
    // };

    let cards = match pagination {
      QueryOperation::None => {
        // Have to be unsafe for our custom types until https://github.com/launchbadge/sqlx/issues/148
        sqlx::query_as_unchecked!(
          Card,
          "SELECT id, category, has_previous, has_next FROM cards_with_pageinfo"
        )
        .fetch_all(&self.pool)
        .await?
      }
      QueryOperation::Before { before } => {
        let before_cursor = Uuid::parse_str(&before.to_string())?;
        // let before_subselect = subselect(before_cursor);

        sqlx::query_as_unchecked!(
          Card,
          "
SELECT id, category, has_previous, has_next
FROM cards_with_pageinfo
WHERE (SELECT sort_order FROM cards_with_pageinfo WHERE id = $1) < sort_order
ORDER BY sort_order ASC
          ",
          before_cursor
        )
        .fetch_all(&self.pool)
        .await?
        // cards_with_pageinfo::table
        //   .filter(
        //     cards_with_pageinfo::sort_order
        //       .nullable()
        //       .lt(before_subselect),
        //   )
        //   .order(cards_with_pageinfo::sort_order.asc())
        //   .load::<Card>(&conn)?
      }
      QueryOperation::After { after } => {
        let after_cursor = Uuid::parse_str(&after.to_string())?;

        sqlx::query_as_unchecked!(
          Card,
          "
SELECT id, category, has_previous, has_next
FROM cards_with_pageinfo
WHERE (SELECT sort_order FROM cards_with_pageinfo WHERE id = $1) > sort_order
ORDER BY sort_order ASC
          ",
          after_cursor
        )
        .fetch_all(&self.pool)
        .await?
        // let after_subselect = subselect(after_cursor);

        // cards_with_pageinfo::table
        //   .filter(
        //     cards_with_pageinfo::sort_order
        //       .nullable()
        //       .gt(after_subselect),
        //   )
        //   .order(cards_with_pageinfo::sort_order.asc())
        //   .load::<Card>(&conn)?
      }
      // QueryOperation::Between { after, before } => {
      //   let after_cursor = Uuid::parse_str(&after.to_string())?;
      //   let before_cursor = Uuid::parse_str(&before.to_string())?;

      //   let after_subselect = subselect(after_cursor);
      //   let before_subselect = subselect(before_cursor);

      //   cards_with_pageinfo::table
      //     .filter(
      //       cards_with_pageinfo::sort_order
      //         .nullable()
      //         .gt(after_subselect),
      //     )
      //     .filter(
      //       cards_with_pageinfo::sort_order
      //         .nullable()
      //         .lt(before_subselect),
      //     )
      //     .order(cards_with_pageinfo::sort_order.asc())
      //     .load::<Card>(&conn)?
      // }
      // QueryOperation::First { limit } => cards_with_pageinfo::table
      //   .order(cards_with_pageinfo::sort_order.asc())
      //   .limit(limit as i64)
      //   .load::<Card>(&conn)?,
      // QueryOperation::FirstAfter { limit, after } => {
      //   let after_cursor = Uuid::parse_str(&after.to_string())?;
      //   let after_subselect = subselect(after_cursor);

      //   cards_with_pageinfo::table
      //     .filter(
      //       cards_with_pageinfo::sort_order
      //         .nullable()
      //         .gt(after_subselect),
      //     )
      //     .order(cards_with_pageinfo::sort_order.asc())
      //     .limit(limit as i64)
      //     .load::<Card>(&conn)?
      // }
      // QueryOperation::FirstBefore { limit, before } => {
      //   let before_cursor = Uuid::parse_str(&before.to_string())?;
      //   let before_subselect = subselect(before_cursor);

      //   cards_with_pageinfo::table
      //     .filter(
      //       cards_with_pageinfo::sort_order
      //         .nullable()
      //         .lt(before_subselect),
      //     )
      //     .order(cards_with_pageinfo::sort_order.asc())
      //     .limit(limit as i64)
      //     .load::<Card>(&conn)?
      // }
      // QueryOperation::FirstBetween {
      //   limit,
      //   before,
      //   after,
      // } => {
      //   let after_cursor = Uuid::parse_str(&after.to_string())?;
      //   let before_cursor = Uuid::parse_str(&before.to_string())?;

      //   let after_subselect = subselect(after_cursor);
      //   let before_subselect = subselect(before_cursor);

      //   cards_with_pageinfo::table
      //     .filter(
      //       cards_with_pageinfo::sort_order
      //         .nullable()
      //         .gt(after_subselect),
      //     )
      //     .filter(
      //       cards_with_pageinfo::sort_order
      //         .nullable()
      //         .lt(before_subselect),
      //     )
      //     .order(cards_with_pageinfo::sort_order.asc())
      //     .limit(limit as i64)
      //     .load::<Card>(&conn)?
      // }
      // QueryOperation::Last { limit } => cards_with_pageinfo::table
      //   .order(cards_with_pageinfo::sort_order.desc())
      //   .limit(limit as i64)
      //   .load::<Card>(&conn)?
      //   .iter()
      //   .cloned()
      //   .rev()
      //   .collect::<Vec<Card>>(),
      // QueryOperation::LastAfter { limit, after } => {
      //   let after_cursor = Uuid::parse_str(&after.to_string())?;
      //   let after_subselect = subselect(after_cursor);

      //   cards_with_pageinfo::table
      //     .filter(
      //       cards_with_pageinfo::sort_order
      //         .nullable()
      //         .gt(after_subselect),
      //     )
      //     .order(cards_with_pageinfo::sort_order.desc())
      //     .limit(limit as i64)
      //     .load::<Card>(&conn)?
      //     .iter()
      //     .cloned()
      //     .rev()
      //     .collect::<Vec<Card>>()
      // }
      // QueryOperation::LastBefore { limit, before } => {
      //   let before_cursor = Uuid::parse_str(&before.to_string())?;
      //   let before_subselect = subselect(before_cursor);

      //   cards_with_pageinfo::table
      //     .filter(
      //       cards_with_pageinfo::sort_order
      //         .nullable()
      //         .lt(before_subselect),
      //     )
      //     .order(cards_with_pageinfo::sort_order.desc())
      //     .limit(limit as i64)
      //     .load::<Card>(&conn)?
      //     .iter()
      //     .cloned()
      //     .rev()
      //     .collect::<Vec<Card>>()
      // }
      // QueryOperation::LastBetween {
      //   limit,
      //   before,
      //   after,
      // } => {
      //   let after_cursor = Uuid::parse_str(&after.to_string())?;
      //   let before_cursor = Uuid::parse_str(&before.to_string())?;

      //   let after_subselect = subselect(after_cursor);
      //   let before_subselect = subselect(before_cursor);

      //   cards_with_pageinfo::table
      //     .filter(
      //       cards_with_pageinfo::sort_order
      //         .nullable()
      //         .gt(after_subselect),
      //     )
      //     .filter(
      //       cards_with_pageinfo::sort_order
      //         .nullable()
      //         .lt(before_subselect),
      //     )
      //     .order(cards_with_pageinfo::sort_order.desc())
      //     .limit(limit as i64)
      //     .load::<Card>(&conn)?
      //     .iter()
      //     .cloned()
      //     .rev()
      //     .collect::<Vec<Card>>()
      // }
      // TODO: How can I error?
      QueryOperation::Invalid => Vec::new(),
      _ => Vec::new(),
    };

    Ok(cards)
  }
}
