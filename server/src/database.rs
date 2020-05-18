use crate::data::battle_card::{BattleCard, BattleCardProps};
use crate::data::character_card::{CharacterCard, CharacterCardProps};
use crate::data::stratagem_card::{StratagemCard, StratagemCardProps};
use crate::data::{Card, CardCategory, CharacterMode, Wave, ID};
use crate::database_schema::{
    battle_cards, cards, cards_with_pageinfo, character_modes, stratagem_cards, waves,
};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

// TODO: this shouldn't be in this file
use async_graphql::QueryOperation;
use uuid::Uuid;

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

    #[allow(dead_code)]
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

    #[allow(dead_code)]
    pub fn get_character_cards(&self) -> Result<Vec<Card>, Error> {
        let conn = self.pool.get()?;

        let cards = cards_with_pageinfo::table
            .filter(cards_with_pageinfo::category.eq(CardCategory::Character))
            .load::<Card>(&conn)?;

        Ok(cards)
    }

    #[allow(dead_code)]
    pub fn get_battle_cards(&self) -> Result<Vec<Card>, Error> {
        let conn = self.pool.get()?;

        let cards = cards_with_pageinfo::table
            .filter(cards_with_pageinfo::category.eq(CardCategory::Battle))
            .load::<Card>(&conn)?;

        Ok(cards)
    }

    #[allow(dead_code)]
    pub fn get_stratagem_cards(&self) -> Result<Vec<Card>, Error> {
        let conn = self.pool.get()?;

        let cards = cards_with_pageinfo::table
            .filter(cards_with_pageinfo::category.eq(CardCategory::Stratagem))
            .load::<Card>(&conn)?;

        Ok(cards)
    }

    pub fn get_cards(&self, pagination: &QueryOperation) -> Result<Vec<Card>, Error> {
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
            QueryOperation::None => cards_with_pageinfo::table.load::<Card>(&conn)?,
            QueryOperation::Before { before } => {
                let before_cursor = Uuid::parse_str(&before.to_string())?;
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
            QueryOperation::After { after } => {
                let after_cursor = Uuid::parse_str(&after.to_string())?;
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
            QueryOperation::Between { after, before } => {
                let after_cursor = Uuid::parse_str(&after.to_string())?;
                let after_subselect = subselect(after_cursor);

                let before_cursor = Uuid::parse_str(&before.to_string())?;
                let before_subselect = subselect(before_cursor);

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
            QueryOperation::First { limit } => cards_with_pageinfo::table
                .order(cards_with_pageinfo::sort_order.asc())
                .limit(*limit as i64)
                .load::<Card>(&conn)?,
            QueryOperation::FirstAfter { limit, after } => {
                let after_cursor = Uuid::parse_str(&after.to_string())?;
                let after_subselect = subselect(after_cursor);

                cards_with_pageinfo::table
                    .filter(
                        cards_with_pageinfo::sort_order
                            .nullable()
                            .gt(after_subselect),
                    )
                    .order(cards_with_pageinfo::sort_order.asc())
                    .limit(*limit as i64)
                    .load::<Card>(&conn)?
            }
            QueryOperation::FirstBefore { limit, before } => {
                let before_cursor = Uuid::parse_str(&before.to_string())?;
                let before_subselect = subselect(before_cursor);

                cards_with_pageinfo::table
                    .filter(
                        cards_with_pageinfo::sort_order
                            .nullable()
                            .lt(before_subselect),
                    )
                    .order(cards_with_pageinfo::sort_order.asc())
                    .limit(*limit as i64)
                    .load::<Card>(&conn)?
            }
            QueryOperation::FirstBetween {
                limit,
                after,
                before,
            } => {
                let before_cursor = Uuid::parse_str(&before.to_string())?;
                let before_subselect = subselect(before_cursor);
                let after_cursor = Uuid::parse_str(&after.to_string())?;
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
                    .limit(*limit as i64)
                    .load::<Card>(&conn)?
            }
            QueryOperation::Last { limit } => cards_with_pageinfo::table
                .order(cards_with_pageinfo::sort_order.desc())
                .limit(*limit as i64)
                .load::<Card>(&conn)?
                .iter()
                .cloned()
                .rev()
                .collect::<Vec<Card>>(),
            QueryOperation::LastAfter { limit, after } => {
                let after_cursor = Uuid::parse_str(&after.to_string())?;
                let after_subselect = subselect(after_cursor);

                cards_with_pageinfo::table
                    .filter(
                        cards_with_pageinfo::sort_order
                            .nullable()
                            .gt(after_subselect),
                    )
                    .order(cards_with_pageinfo::sort_order.desc())
                    .limit(*limit as i64)
                    .load::<Card>(&conn)?
                    .iter()
                    .cloned()
                    .rev()
                    .collect::<Vec<Card>>()
            }
            QueryOperation::LastBefore { limit, before } => {
                let before_cursor = Uuid::parse_str(&before.to_string())?;
                let before_subselect = subselect(before_cursor);

                cards_with_pageinfo::table
                    .filter(
                        cards_with_pageinfo::sort_order
                            .nullable()
                            .lt(before_subselect),
                    )
                    .order(cards_with_pageinfo::sort_order.desc())
                    .limit(*limit as i64)
                    .load::<Card>(&conn)?
                    .iter()
                    .cloned()
                    .rev()
                    .collect::<Vec<Card>>()
            }
            QueryOperation::LastBetween {
                limit,
                after,
                before,
            } => {
                let before_cursor = Uuid::parse_str(&before.to_string())?;
                let before_subselect = subselect(before_cursor);
                let after_cursor = Uuid::parse_str(&after.to_string())?;
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
                    .limit(*limit as i64)
                    .load::<Card>(&conn)?
                    .iter()
                    .cloned()
                    .rev()
                    .collect::<Vec<Card>>()
            }
            // TODO: How can I error?
            QueryOperation::Invalid => Vec::new(),
        };

        Ok(cards.into_iter().map(|card| card.into()).collect())
    }
}

// These need to go away
impl Database {
    pub fn load_battle_card(&self, card: Card) -> Result<BattleCard, Error> {
        let conn = self.pool.get()?;

        let battle_card = battle_cards::table
            .filter(battle_cards::card_id.eq(card.id))
            .first::<BattleCardProps>(&conn)
            // TODO: performance of cloning this?
            .map(|extra| BattleCard::new(card.clone(), extra))?;

        Ok(battle_card)
    }

    pub fn load_character_card(&self, card: Card) -> Result<CharacterCard, Error> {
        let conn = self.pool.get()?;

        let character_card = character_modes::table
            .filter(character_modes::card_id.eq(card.id))
            .load::<CharacterMode>(&conn)
            // TODO: performance of cloning this?
            .map(|modes| CharacterCard::new(card.clone(), CharacterCardProps { modes }))?;

        Ok(character_card)
    }

    pub fn load_stratagem_card(&self, card: Card) -> Result<StratagemCard, Error> {
        let conn = self.pool.get()?;

        let stratagem_card = stratagem_cards::table
            .filter(stratagem_cards::card_id.eq(card.id))
            .first::<StratagemCardProps>(&conn)
            // TODO: performance of cloning this?
            .map(|extra| StratagemCard::new(card.clone(), extra))?;

        Ok(stratagem_card)
    }
}
