use crate::data::{Card, CardCategory, CardRarity, CharacterMode, Wave, ID};
use crate::database::{get_wave, ConnPool};
use crate::database_schema::character_modes;
use crate::schema::Cursor;
use async_graphql::{Context, FieldResult};
use diesel::prelude::*;
use tokio_diesel::*;

#[derive(Clone, Debug)]
pub struct ExtraProps {
  modes: Vec<CharacterMode>,
}

#[derive(Clone, Debug)]
pub struct CharacterCard(Card, ExtraProps);

impl CharacterCard {
  pub fn new(card: Card, modes: ExtraProps) -> Self {
    CharacterCard(card, modes)
  }

  pub async fn load_from_card(card: Card, pool: &ConnPool) -> AsyncResult<CharacterCard> {
    character_modes::table
      .filter(character_modes::card_id.eq(card.id))
      .load_async::<CharacterMode>(&pool)
      .await
      // TODO: performance of cloning this?
      .map(|modes| CharacterCard::new(card.clone(), ExtraProps { modes }))
  }
}

#[async_graphql::Object]
impl CharacterCard {
  pub fn cursor(&self) -> Cursor {
    Cursor::new(self.0.id)
  }

  #[field]
  pub async fn id(&self) -> ID {
    self.0.id
  }

  #[field]
  pub async fn tcg_id(&self) -> &str {
    &self.0.tcg_id
  }

  #[field]
  pub async fn rarity(&self) -> CardRarity {
    self.0.rarity
  }

  #[field]
  pub async fn number(&self) -> &str {
    &self.0.number
  }

  #[field]
  pub async fn category(&self) -> CardCategory {
    self.0.category
  }

  #[field]
  pub async fn wave(&self, ctx: &Context<'_>) -> FieldResult<Wave> {
    let pool = ctx.data::<ConnPool>();
    let wave = get_wave(pool, self.0.wave_id).await?;
    Ok(wave)
  }

  #[field]
  pub async fn modes(&self) -> &Vec<CharacterMode> {
    &self.1.modes
  }
}
