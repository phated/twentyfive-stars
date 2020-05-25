use crate::data::{BattleCard, CharacterCard, NodeType, StratagemCard};
use crate::graphql_schema::ContextData;

#[async_graphql::Union]
#[derive(Debug, Clone)]
pub enum Card {
    Battle(BattleCard),
    Character(CharacterCard),
    Stratagem(StratagemCard),
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
            let data = ctx.data::<ContextData>();
            let card_nodes = data.db.get_card_nodes().await?;

            // let has_previous = cards.first().map_or(false, |card| card.has_previous);
            let has_previous = false;
            // let has_next = cards.last().map_or(false, |card| card.has_next);
            let has_next = false;
            let mut nodes = vec![];

            for card_node in card_nodes {
                match card_node.node_type {
                    NodeType::Battle => {
                        let cursor = card_node.node_id.into();
                        let node = data.db.get_battle_card(card_node.id).await?.into();
                        nodes.push((cursor, EmptyEdgeFields, node))
                    }
                    _ => todo!(),
                }
            }

            let connection = Connection::new(None, has_previous, has_next, nodes);
            Ok(connection)
        }
    }
}
