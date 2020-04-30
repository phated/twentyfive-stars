use crate::data::{Card, CardCategory, CardRarity, CharacterMode, Wave, ID};
use crate::database::{get_wave, ConnPool};
use crate::database_schema::character_modes;
use crate::schema::Cursor;
use async_graphql::{Context, FieldResult};
use diesel::prelude::*;

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

  pub fn load_from_card(card: Card, pool: &ConnPool) -> QueryResult<CharacterCard> {
    let conn = pool.get().unwrap();

    character_modes::table
      .filter(character_modes::card_id.eq(card.id))
      .load::<CharacterMode>(&conn)
      // TODO: performance of cloning this?
      .map(|modes| CharacterCard::new(card.clone(), ExtraProps { modes }))
  }
}

#[async_graphql::Object]
impl CharacterCard {
  #[field(skip)]
  pub fn cursor(&self) -> Cursor {
    Cursor::new(self.0.id)
  }

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
    let pool = ctx.data::<ConnPool>();
    let wave = get_wave(pool, self.0.wave_id)?;
    Ok(wave)
  }

  pub async fn modes(&self) -> &Vec<CharacterMode> {
    &self.1.modes
  }
}
