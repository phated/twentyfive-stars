use crate::data::{Card, CardCategory, CardRarity, CharacterMode, Wave, ID};
use crate::database::Database;
use async_graphql::{Context, Cursor, FieldResult};

#[derive(Clone, Debug)]
pub struct CharacterCardProps {
  pub modes: Vec<CharacterMode>,
}

#[derive(Clone, Debug)]
pub struct CharacterCard(Card, CharacterCardProps);

impl CharacterCard {
  pub fn new(card: Card, extra: CharacterCardProps) -> Self {
    CharacterCard(card, extra)
  }
}

impl Into<Cursor> for CharacterCard {
  fn into(self) -> Cursor {
    self.0.id.into()
  }
}

#[async_graphql::Object]
impl CharacterCard {
  pub async fn id(&self) -> ID {
    self.0.id
  }

  pub async fn tcg_id(&self) -> &str {
    &self.0.tcg_id
  }

  pub async fn rarity(&self) -> CardRarity {
    self.0.rarity
  }

  pub async fn number(&self) -> &str {
    &self.0.number
  }

  pub async fn category(&self) -> CardCategory {
    self.0.category
  }

  pub async fn wave(&self, ctx: &Context<'_>) -> FieldResult<Wave> {
    let db = ctx.data::<Database>();
    let wave = db.get_wave(self.0.wave_id)?;
    Ok(wave)
  }

  pub async fn modes(&self) -> &Vec<CharacterMode> {
    &self.1.modes
  }
}
