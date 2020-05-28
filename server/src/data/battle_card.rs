use crate::data::{BattleIcon, BattleType, CardCategory, CardRarity, Faction, Wave};
use crate::graphql_schema::ContextData;
use async_graphql::{Context, FieldResult};
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct BattleCard {
    // Generic card props
    pub id: i32,
    pub node_id: Uuid,
    pub tcg_id: String,
    pub rarity: CardRarity,
    pub number: String,
    pub category: CardCategory,
    // Battle card specific props
    pub title: String,
    pub stars: Option<i32>,
    // pub icons: Vec<BattleIcon>,
    pub r#type: BattleType,
    pub faction: Option<Faction>,
    pub attack_modifier: Option<i32>,
    pub defense_modifier: Option<i32>,
}

#[async_graphql::Object]
impl BattleCard {
    pub async fn id(&self) -> async_graphql::ID {
        self.node_id.into()
    }

    pub async fn tcg_id(&self) -> &str {
        &self.tcg_id
    }

    pub async fn rarity(&self) -> CardRarity {
        self.rarity
    }

    pub async fn number(&self) -> &str {
        &self.number
    }

    pub async fn category(&self) -> CardCategory {
        self.category
    }

    pub async fn wave(&self, ctx: &Context<'_>) -> FieldResult<Wave> {
        let data = ctx.data::<ContextData>();
        let wave = data.db.get_wave_for_battle_card(self).await?;

        Ok(wave)
    }

    pub async fn title(&self) -> &str {
        &self.title
    }

    pub async fn stars(&self) -> Option<i32> {
        self.stars
    }

    pub async fn icons(&self) -> Vec<BattleIcon> {
        // self.icons
        vec![]
    }

    pub async fn type_(&self) -> BattleType {
        self.r#type
    }

    pub async fn faction(&self) -> Option<Faction> {
        self.faction
    }

    pub async fn attack_modifier(&self) -> Option<i32> {
        self.attack_modifier
    }

    pub async fn defense_modifier(&self) -> Option<i32> {
        self.defense_modifier
    }
}
