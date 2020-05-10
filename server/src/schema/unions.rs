use crate::data::{BattleCard, CharacterCard, StratagemCard};
use crate::database;
use async_graphql::{
  Connection, Context, DataSource, EmptyEdgeFields, FieldResult, QueryOperation,
};

#[async_graphql::Union]
pub struct Card(BattleCard, CharacterCard, StratagemCard);

// TODO: I'm not sure what to do with this
pub struct Cards;

#[DataSource]
impl DataSource for Cards {
  type Element = Card;

  type EdgeFieldsObj = EmptyEdgeFields;

  async fn query_operation(
    &self,
    ctx: &Context<'_>,
    operation: &QueryOperation,
  ) -> FieldResult<Connection<Self::Element, Self::EdgeFieldsObj>> {
    let db = ctx.data::<database::Database>();
    // let pagination = Pagination::new(first, last, before, after);
    // TODO: datasource handles this
    let pagination = QueryOperation::None;
    let cards: Vec<crate::data::Card> = db.get_cards(pagination).await?;

    let has_previous = cards.first().map_or(false, |card| card.has_previous);
    let has_next = cards.last().map_or(false, |card| card.has_next);
    let mut nodes = vec![];

    for card in cards {
      nodes.push((card.id.into(), EmptyEdgeFields, dbg!(card).into()));
      // match card.category {
      //   CardCategory::Battle => {
      //     let battle_card = BattleCard::load_from_card(card, pool)?;
      //     nodes.push((
      //       battle_card.cursor().into(),
      //       EmptyEdgeFields,
      //       battle_card.into(),
      //     ))
      //   }
      //   CardCategory::Character => {
      //     let character_card = CharacterCard::load_from_card(card, pool)?;
      //     nodes.push((
      //       character_card.cursor().into(),
      //       EmptyEdgeFields,
      //       character_card.into(),
      //     ))
      //   }
      //   CardCategory::Stratagem => {
      //     let stratagem_card = StratagemCard::load_from_card(card, pool)?;
      //     nodes.push((
      //       stratagem_card.cursor().into(),
      //       EmptyEdgeFields,
      //       stratagem_card.into(),
      //     ))
      //   }
      // }
    }

    let connection = Connection::new(None, has_previous, has_next, nodes);
    Ok(connection)
  }
}
