use crate::data::{Card, CardCategory, CardRarity, CharacterMode, Wave};
use crate::database_schema::character_modes;
use crate::graphql_schema::Context;
use diesel::prelude::*;
use juniper::FieldResult;
use uuid::Uuid;

pub struct ExtraProps {
  modes: Vec<CharacterMode>,
}

pub struct CharacterCard(Card, ExtraProps);

impl CharacterCard {
  pub fn new(card: Card, modes: ExtraProps) -> Self {
    CharacterCard(card, modes)
  }

  pub fn load_from_card(card: &Card, context: &Context) -> Option<Self> {
    character_modes::table
      .filter(character_modes::card_id.eq(card.id))
      .load::<CharacterMode>(&context.connection)
      .ok()
      // TODO: performance of cloning this?
      .map(|modes| CharacterCard::new(card.clone(), ExtraProps { modes }))
  }
}

impl CharacterCard {
  pub fn id(&self) -> Uuid {
    match self {
      CharacterCard(card, _extra) => card.id(),
    }
  }

  pub fn tcg_id(&self) -> &str {
    match self {
      CharacterCard(card, _extra) => card.tcg_id(),
    }
  }

  pub fn rarity(&self) -> &CardRarity {
    match self {
      CharacterCard(card, _extra) => card.rarity(),
    }
  }

  pub fn number(&self) -> &str {
    match self {
      CharacterCard(card, _extra) => card.number(),
    }
  }

  pub fn category(&self) -> &CardCategory {
    match self {
      CharacterCard(card, _extra) => card.category(),
    }
  }

  pub fn wave(&self, context: &Context) -> QueryResult<Wave> {
    match self {
      CharacterCard(card, _extra) => card.wave(context),
    }
  }

  pub fn modes(&self) -> &Vec<CharacterMode> {
    match self {
      CharacterCard(_card, extra) => &extra.modes,
    }
  }
}

juniper::graphql_object!(CharacterCard: Context | &self | {
  interfaces: [&Card]

  field id() -> Uuid {
    self.id()
  }

  field tcg_id() -> &str {
    self.tcg_id()
  }

  field rarity() -> &CardRarity {
    self.rarity()
  }

  field number() -> &str {
    self.number()
  }

  field category() -> &CardCategory {
    self.category()
  }

  field wave(&executor) -> FieldResult<Wave> {
    let context = executor.context();
    // TODO: weird conversion between result types
    let wave = self.wave(context)?;
    Ok(wave)
  }

  field modes() -> &Vec<CharacterMode> {
    self.modes()
  }
});
