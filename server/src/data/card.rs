use crate::data::{CardCategory, CardRarity, ID};
use crate::database;
use crate::database_schema::cards_with_pageinfo;

#[derive(Identifiable, Queryable, Clone, PartialEq, Eq, Debug)]
#[table_name = "cards_with_pageinfo"]
pub struct Card {
  pub id: ID,
  pub tcg_id: String,
  pub rarity: CardRarity,
  pub number: String,
  pub category: CardCategory,
  pub wave_id: ID,
  pub sort_order: i32,
  pub has_previous: bool,
  pub has_next: bool,
}

pub mod datasource {
  use super::*;
  use crate::schema::interfaces;
  use async_graphql::{
    Connection, Context, DataSource, EmptyEdgeFields, FieldResult, QueryOperation,
  };

  pub struct CardDataSource;

  #[async_trait::async_trait]
  impl DataSource for CardDataSource {
    type Element = interfaces::Card;
    type EdgeFieldsObj = EmptyEdgeFields;

    async fn query_operation(
      &self,
      ctx: &Context<'_>,
      pagination: &QueryOperation,
    ) -> FieldResult<Connection<Self::Element, Self::EdgeFieldsObj>> {
      let db = ctx.data::<database::Database>();
      let cards = db.get_cards(pagination)?;

      let has_previous = cards.first().map_or(false, |card| card.has_previous);
      let has_next = cards.last().map_or(false, |card| card.has_next);
      let mut nodes = vec![];

      for card in cards {
        match card.category {
          CardCategory::Battle => {
            let battle_card = db.load_battle_card(card)?;
            // The .into() takes ownership so we can't convert to the interface
            let cursor = battle_card.clone().into();
            let node = battle_card.into();
            nodes.push((cursor, EmptyEdgeFields, node))
          }
          CardCategory::Character => {
            let character_card = db.load_character_card(card)?;
            // The .into() takes ownership so we can't convert to the interface
            let cursor = character_card.clone().into();
            let node = character_card.clone().into();
            nodes.push((cursor, EmptyEdgeFields, node))
          }
          CardCategory::Stratagem => {
            let stratagem_card = db.load_stratagem_card(card)?;
            // The .into() takes ownership so we can't convert to the interface
            let cursor = stratagem_card.clone().into();
            let node = stratagem_card.into();
            nodes.push((cursor, EmptyEdgeFields, node))
          }
        }
      }

      let connection = Connection::new(None, has_previous, has_next, nodes);
      Ok(connection)
    }
  }
}
