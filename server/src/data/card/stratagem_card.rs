use crate::data::{CardCategory, CardRarity, Faction, Wave};
use crate::graphql_schema::ContextData;
use async_graphql::{Context, FieldResult};
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct StratagemCard {
    // Generic card props
    pub id: i32,
    pub node_id: Uuid,
    pub tcg_id: String,
    pub rarity: CardRarity,
    pub number: String,
    pub category: CardCategory,
    // Stratagem card specific props
    pub title: String,
    // TODO: It would be cool to reach into the graph
    // for the characters tied to this requirement
    pub requirement: String,
    pub stars: i32,
    pub faction: Faction,
}

#[async_graphql::Object]
impl StratagemCard {
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
        let data = ctx.data::<ContextData>()?;
        let wave = data.db.get_wave_for_stratagem_card(self).await?;

        Ok(wave)
    }

    pub async fn title(&self) -> &str {
        &self.title
    }

    pub async fn requirement(&self) -> &str {
        &self.requirement
    }

    pub async fn stars(&self) -> i32 {
        self.stars
    }

    pub async fn faction(&self) -> Faction {
        self.faction
    }
}
