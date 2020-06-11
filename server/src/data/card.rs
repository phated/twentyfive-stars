use crate::data::NodeType;
use crate::graphql_schema::ContextData;

pub mod battle_card;
pub mod character_card;
pub mod stratagem_card;

pub use battle_card::*;
pub use character_card::*;
pub use stratagem_card::*;

#[async_graphql::Union]
#[derive(Debug, Clone)]
pub enum Cards {
    Battle(BattleCard),
    Character(CharacterCard),
    Stratagem(StratagemCard),
}

pub mod datasource {
    use super::*;
    use async_graphql::connection::{Connection, DataSource, Edge, EmptyFields};
    use async_graphql::{Context, FieldResult};

    pub struct CardDataSource;

    #[async_graphql::DataSource]
    impl DataSource for CardDataSource {
        type CursorType = String;
        type NodeType = Cards;
        type ConnectionFieldsType = EmptyFields;
        type EdgeFieldsType = EmptyFields;

        async fn execute_query(
            &self,
            ctx: &Context<'_>,
            after: Option<Self::CursorType>,
            before: Option<Self::CursorType>,
            first: Option<usize>,
            last: Option<usize>,
        ) -> FieldResult<
            Connection<
                Self::CursorType,
                Self::NodeType,
                Self::ConnectionFieldsType,
                Self::EdgeFieldsType,
            >,
        > {
            let data = ctx.data::<ContextData>();
            let card_nodes = data.db.get_card_nodes().await?;

            let start_idx = after
                .and_then(|cursor| {
                    card_nodes
                        .clone()
                        .into_iter()
                        .position(|node| node.node_id.to_string() == cursor)
                        .map(|idx| idx + 1)
                })
                .unwrap_or(0);
            let end_idx = before
                .and_then(|cursor| {
                    card_nodes
                        .clone()
                        .into_iter()
                        .rposition(|node| node.node_id.to_string() == cursor)
                })
                .unwrap_or(card_nodes.len());

            let mut has_previous_page = start_idx > 0;
            let mut has_next_page = end_idx < card_nodes.len();

            let mut nodes = &card_nodes[start_idx..end_idx];

            if let Some(first) = first {
                if nodes.len() > first {
                    let slice_begin = 0;
                    let slice_end = first;
                    nodes = &nodes[slice_begin..slice_end];
                    // TODO: need to check this
                    has_next_page = true;
                }
            }

            if let Some(last) = last {
                if nodes.len() > last {
                    let slice_begin = nodes.len() - last;
                    let slice_end = nodes.len();
                    nodes = &nodes[slice_begin..slice_end];
                    // TODO: need to check this
                    has_previous_page = true;
                }
            }

            let mut edges = vec![];

            for node in nodes {
                let edge = match node.node_type {
                    NodeType::Battle => {
                        let node = data.db.get_battle_card(node.id).await?;
                        let cursor = node.node_id.to_string().into();
                        Edge::new(cursor, node.into())
                    }
                    NodeType::Character => {
                        let node = data.db.get_character_card(node.id).await?;
                        let cursor = node.node_id.to_string().into();
                        Edge::new(cursor, node.into())
                    }
                    _ => todo!(),
                };
                edges.push(edge);
            }

            let mut connection = Connection::new(has_previous_page, has_next_page);
            connection.append(edges);

            Ok(connection)
        }
    }
}
