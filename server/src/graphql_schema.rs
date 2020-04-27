use crate::data::card::schema::Card;
use crate::data::{BattleCard, CardCategory, CharacterCard, StratagemCard};
use crate::database;
use crate::pagination::Pagination;
use crate::schema::Cursor;
use async_graphql::{Connection, Context, EmptyEdgeFields, FieldResult};

pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
  // fn node(context: &Context, id: ID) -> FieldResult<Node> {
  //   let node = database::get_node(&context.connection, id)?;
  //   Ok(node)
  // }

  async fn all_cards(
    &self,
    ctx: &Context<'_>,
    after: Option<Cursor>,
    before: Option<Cursor>,
    first: Option<i32>,
    last: Option<i32>,
  ) -> FieldResult<Connection<Card, EmptyEdgeFields>> {
    let pool = ctx.data::<database::ConnPool>();
    let pagination = Pagination::new(first, last, before, after);
    let cards = database::get_cards(&pool, pagination).await?;

    let has_previous = cards.first().map_or(false, |card| card.has_previous);
    let has_next = cards.last().map_or(false, |card| card.has_next);
    let mut nodes = vec![];

    for card in cards {
      match card.category {
        CardCategory::Battle => {
          let battle_card = BattleCard::load_from_card(card, pool).await?;
          nodes.push((
            battle_card.cursor().into(),
            EmptyEdgeFields,
            battle_card.into(),
          ))
        }
        CardCategory::Character => {
          let character_card = CharacterCard::load_from_card(card, pool).await?;
          nodes.push((
            character_card.cursor().into(),
            EmptyEdgeFields,
            character_card.into(),
          ))
        }
        CardCategory::Stratagem => {
          let stratagem_card = StratagemCard::load_from_card(card, pool).await?;
          nodes.push((
            stratagem_card.cursor().into(),
            EmptyEdgeFields,
            stratagem_card.into(),
          ))
        }
      }
    }
    // let battle_cards = cards
    //   .into_iter()
    //   .map(|card| )
    //   .collect::<Vec<_>>();

    // let b = join_all(battle_cards).await;

    // let nodes = battle_cards
    //   .into_iter()
    //   .map(|card| match card {
    //     Some(card) => Some(),
    //     None => None,
    //   })
    //   .collect::<Vec<(String, EmptyEdgeFields, Card)>>();
    // let has_previous = nodes
    //   .first()
    //   .map_or(false, |(_, _, node)| node.clone().has_previous());
    // let has_next = nodes
    //   .last()
    //   .map_or(false, |(_, _, node)| node.clone().has_next());
    let connection = Connection::new(None, has_previous, has_next, nodes);
    Ok(connection)
  }

  // fn all_character_cards(context: &Context) -> FieldResult<Vec<CharacterCard>> {
  //   let cards = database::get_character_cards(&context.connection)?
  //     .iter()
  //     .filter_map(|card| CharacterCard::load_from_card(card, context))
  //     .collect();
  //   // TODO: weird conversion between result types
  //   Ok(cards)
  // }

  // fn all_battle_cards(context: &Context) -> FieldResult<Vec<BattleCard>> {
  //   let cards = database::get_battle_cards(&context.connection)?
  //     .iter()
  //     .filter_map(|card| BattleCard::load_from_card(card, context))
  //     .collect();
  //   // TODO: weird conversion between result types
  //   Ok(cards)
  // }

  // fn all_stratagem_cards(context: &Context) -> FieldResult<Vec<StratagemCard>> {
  //   let cards = database::get_stratagem_cards(&context.connection)?
  //     .iter()
  //     .filter_map(|card| StratagemCard::load_from_card(card, context))
  //     .collect();
  //   // TODO: weird conversion between result types
  //   Ok(cards)
  // }
}
