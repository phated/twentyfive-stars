use crate::auth::{Permission, PermissionGuard};
use crate::data::{BattleCard, BattleCardInput};
use crate::data::{Cards, NodeType, Wave, WaveInput};
use crate::database::Database;
use crate::schema::interfaces;
use async_graphql::connection::{Connection, EmptyFields};
use async_graphql::guard::Guard;
use async_graphql::{Context, EmptySubscription, FieldResult, ID};
use std::convert::TryFrom;
use uuid::Uuid;

pub type Schema = async_graphql::Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub struct QueryRoot;
pub struct MutationRoot;

pub struct ContextData {
    pub db: Database,
}

#[async_graphql::Object]
impl QueryRoot {
    pub async fn node(&self, ctx: &Context<'_>, id: ID) -> FieldResult<interfaces::Node> {
        let data = ctx.data::<ContextData>();
        let node_id = Uuid::try_from(id)?;
        let node = data.db.get_node_by_uuid(node_id).await?;

        match node.node_type {
            NodeType::Battle => {
                let battle_card = data.db.get_battle_card(node.id).await?;

                Ok(battle_card.into())
            }
            NodeType::Character => {
                let character_card = data.db.get_character_card(node.id).await?;

                Ok(character_card.into())
            }
            NodeType::Stratagem => {
                let stratagem_card = data.db.get_stratagem_card(node.id).await?;

                Ok(stratagem_card.into())
            }
            NodeType::Wave => {
                let wave = data.db.get_wave(node.id).await?;

                Ok(wave.into())
            }
            _ => todo!(),
        }
    }

    pub async fn all_cards(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> FieldResult<Connection<String, Cards, EmptyFields, EmptyFields>> {
        Cards::query(ctx, after, before, first, last).await
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

#[async_graphql::Object]
impl MutationRoot {
    #[field(guard(PermissionGuard(permission = "Permission::CreateWaves")))]
    pub async fn add_wave(&self, ctx: &Context<'_>, wave: WaveInput) -> FieldResult<Wave> {
        let data = ctx.data::<ContextData>();
        let result = data.db.create_wave(wave).await;

        match result {
            Ok(wave) => Ok(wave),
            Err(_) => {
                // TODO: why doesn't this error print?
                Err("Unable to add wave".into())
            }
        }
    }

    #[field(guard(PermissionGuard(permission = "Permission::CreateBattleCards")))]
    pub async fn add_battle_card(
        &self,
        ctx: &Context<'_>,
        card: BattleCardInput,
    ) -> FieldResult<BattleCard> {
        let data = ctx.data::<ContextData>();
        let result = data.db.create_battle_card(card).await;

        match result {
            Ok(card) => Ok(card),
            Err(_) => {
                // TODO: why doesn't this error print?
                Err("Unable to add battle card".into())
            }
        }
    }
}
