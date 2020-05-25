use crate::data::CardDataSource;
use crate::database::Database;
use crate::schema::interfaces;
use async_graphql::{Connection, Context, Cursor, DataSource, EmptyEdgeFields, FieldResult};
pub struct QueryRoot;

pub struct ContextData {
    pub db: Database,
}

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
    ) -> FieldResult<Connection<interfaces::Card, EmptyEdgeFields>> {
        CardDataSource.query(ctx, after, before, first, last).await
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
