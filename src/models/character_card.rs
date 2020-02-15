use crate::database::schema::{cards, character_modes, waves};
use crate::database::types;
use crate::database::types::{CardCategory, CardRarity, CharacterTrait, ModeType};
use crate::graphql_schema::Context;
use crate::models::{AltMode, Card, CharacterMode, Wave};
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};
use uuid::Uuid;

pub struct CharacterCard(types::Card, Wave, Vec<types::CharacterMode>);

impl CharacterCard {
  pub fn new(card: types::Card, wave: Wave, modes: Vec<types::CharacterMode>) -> CharacterCard {
    CharacterCard(card, wave, modes)
  }

  pub fn id(&self) -> Uuid {
    match self {
      CharacterCard(card, _wave, _modes) => card.id,
    }
  }

  pub fn tcg_id(&self) -> &str {
    match self {
      CharacterCard(card, _wave, _modes) => &card.tcg_id,
    }
  }

  pub fn rarity(&self) -> &CardRarity {
    match self {
      CharacterCard(card, _wave, _modes) => &card.rarity,
    }
  }

  pub fn number(&self) -> &str {
    match self {
      CharacterCard(card, _wave, _modes) => &card.number,
    }
  }

  pub fn category(&self) -> &CardCategory {
    match self {
      CharacterCard(card, _wave, _modes) => &card.category,
    }
  }

  pub fn wave(&self) -> Wave {
    match self {
      CharacterCard(_card, wave, _modes) => Wave {
        id: wave.id,
        tcg_id: wave.tcg_id.clone(),
        name: wave.name.clone(),
        released: wave.released,
      },
    }
  }

  // pub fn wave(&self, context: &Context) -> Wave {
  //   waves::table
  //     .inner_join(cards::table)
  //     .select((waves::id, waves::tcg_id, waves::name, waves::released))
  //     .first::<Wave>(&context.connection)
  //     .expect("Error loading wave")
  // }

  pub fn modes(&self) -> Vec<CharacterMode> {
    match self {
      CharacterCard(_card, _wave, modes) => modes.iter().map(CharacterMode::from).collect(),
    }
  }
  // pub fn modes(&self, context: &Context) -> Vec<CharacterMode> {
  //   character_modes::table
  //     .filter(character_modes::card_id.eq(self.id))
  //     // .select()
  //     .load::<(
  //       Uuid,
  //       Uuid,
  //       String,
  //       Option<String>,
  //       i32,
  //       ModeType,
  //       CharacterTrait,
  //       i32,
  //       i32,
  //       i32,
  //     )>(&context.connection)
  //     .expect("error loading modes")
  //     .into_iter()
  //     .map(|card| card.0)
  //     .collect()
  // }
}

juniper::graphql_object!(CharacterCard: Context | &self | {
  interfaces: [&Card]

  field id() -> Uuid {
    self.id()
  }

  // field card_id() -> Uuid {
  //   self.card_id()
  // }

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

  // field wave(&executor) -> Wave {
  //   self.wave(executor.context())
  // }

  field wave() -> Wave {
    self.wave()
  }

  field modes() -> Vec<CharacterMode> {
    self.modes()
  }
  // field modes(&executor) -> Vec<CharacterMode> {
  //   self.modes(executor.context())
  // }
});
