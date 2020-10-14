use crate::data::{BattleType, CardCategory, CardRarity, Faction, Image, ImageInput, Wave};
use crate::graphql_schema::ContextData;
use async_graphql::{Context, FieldResult, InputObject, ID};
use uuid::Uuid;

#[derive(Debug, Clone, InputObject)]
pub struct BattleCardInput {
    pub tcg_id: String,
    pub rarity: CardRarity,
    pub number: String,
    pub title: String,
    pub stars: Option<i32>,
    // TODO: Vec<BattleIcon>
    pub icons: Vec<String>,
    pub type_: BattleType,
    pub faction: Option<Faction>,
    pub attack_modifier: Option<i32>,
    pub defense_modifier: Option<i32>,
    pub wave_tcg_id: String,
    pub image: ImageInput,
}

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
    // TODO: Vec<BattleIcon>,
    pub icons: Vec<String>,
    pub r#type: BattleType,
    pub faction: Option<Faction>,
    pub attack_modifier: Option<i32>,
    pub defense_modifier: Option<i32>,
    pub image_id: Option<i32>,
}

#[async_graphql::Object]
impl BattleCard {
    pub async fn id(&self) -> ID {
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
        let wave = data.db.get_wave_for_battle_card(self).await?;

        Ok(wave)
    }

    pub async fn title(&self) -> &str {
        &self.title
    }

    pub async fn stars(&self) -> Option<i32> {
        self.stars
    }

    // TODO: Vec<BattleIcon>
    pub async fn icons(&self) -> Vec<String> {
        self.icons.clone()
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

    pub async fn image(&self, ctx: &Context<'_>) -> FieldResult<Option<Image>> {
        let data = ctx.data::<ContextData>()?;
        let image = match self.image_id {
            Some(image_id) => {
                let result = data.db.get_image(image_id).await;

                match result {
                    Ok(image) => Some(image),
                    Err(_) => {
                        // TODO: log error
                        None
                    }
                }
            }
            None => None,
        };
        Ok(image)
    }
}
