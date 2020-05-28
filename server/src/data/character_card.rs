use crate::data::{CardCategory, CardRarity, CharacterMode, Wave};
use crate::graphql_schema::ContextData;
use async_graphql::{Context, FieldResult};
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct CharacterCard {
    // Generic card props
    pub id: i32,
    pub node_id: Uuid,
    pub tcg_id: String,
    pub rarity: CardRarity,
    pub number: String,
    pub category: CardCategory,
}

#[async_graphql::Object]
impl CharacterCard {
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
        let wave = data.db.get_wave_for_character_card(self).await?;

        Ok(wave)
    }

    pub async fn modes(&self, ctx: &Context<'_>) -> FieldResult<Vec<CharacterMode>> {
        let data = ctx.data::<ContextData>();
        let modes = data.db.get_modes_for_character_card(self).await?;

        Ok(modes)
    }
}
