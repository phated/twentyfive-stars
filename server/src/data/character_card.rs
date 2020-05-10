use crate::data::{Card, CardCategory, CardRarity, CharacterMode, Wave, ID};
use crate::database::Database;
use crate::database_schema::character_modes;
use async_graphql::{Context, Cursor, FieldResult};
use diesel::prelude::*;

#[derive(Clone, Debug)]
pub struct CharacterCard {
  // Base Card
  pub id: ID,
  pub tcg_id: String,
  pub rarity: CardRarity,
  pub number: String,
  pub category: CardCategory,
  pub wave_id: ID,
  pub sort_order: i32,
  pub has_previous: bool,
  pub has_next: bool,
  // Character Card
  pub modes: Vec<CharacterMode>,
}

impl CharacterCard {
  // pub fn new(card: Card, modes: ExtraProps) -> Self {
  //   CharacterCard(card, modes)
  // }

  // pub fn load_from_card(card: Card, pool: &ConnPool) -> QueryResult<CharacterCard> {
  //   let conn = pool.get().unwrap();

  //   character_modes::table
  //     .filter(character_modes::card_id.eq(card.id))
  //     .load::<CharacterMode>(&conn)
  //     // TODO: performance of cloning this?
  //     .map(|modes| CharacterCard::new(card.clone(), ExtraProps { modes }))
  // }
}

impl Into<Cursor> for CharacterCard {
  fn into(self) -> Cursor {
    self.id.into()
  }
}

#[async_graphql::Object]
impl CharacterCard {
  pub async fn id(&self) -> ID {
    self.id
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
    let db = ctx.data::<Database>();
    let wave = db.get_wave(self.wave_id).await?;
    Ok(wave)
  }

  pub async fn modes(&self) -> &Vec<CharacterMode> {
    &self.modes
  }
}
