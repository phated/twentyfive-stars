use crate::database::types::{CardCategory, CardRarity};
use crate::graphql_schema::Context;
use crate::models::Wave;
use crate::models::{BattleCard, CharacterCard};
use uuid::Uuid;

pub enum Card {
  BattleCard(BattleCard),
  CharacterCard(CharacterCard),
}

impl From<BattleCard> for Card {
  fn from(card: BattleCard) -> Self {
    Card::BattleCard(card)
  }
}

impl From<CharacterCard> for Card {
  fn from(card: CharacterCard) -> Self {
    Card::CharacterCard(card)
  }
}

juniper::graphql_interface!(Card: Context |&self| {
  field id() -> Uuid {
    match *self {
      Card::BattleCard(BattleCard { id, .. }) => id,
      Card::CharacterCard(ref card) => card.id(),
    }
  }

  field tcg_id() -> &str {
    match *self {
      Card::BattleCard(BattleCard { ref tcg_id, .. }) => tcg_id,
      Card::CharacterCard(ref card) => card.tcg_id(),
    }
  }

  field rarity() -> &CardRarity {
    match *self {
      Card::BattleCard(BattleCard { ref rarity, .. }) => rarity,
      Card::CharacterCard(ref card) => card.rarity(),
    }
  }

  field number() -> &str {
    match *self {
      Card::BattleCard(BattleCard { ref number, .. }) => number,
      Card::CharacterCard(ref card) => card.number(),
    }
  }

  field category() -> &CardCategory {
    match *self {
      Card::BattleCard(BattleCard { ref category, .. }) => category,
      Card::CharacterCard(ref card) => card.category(),
    }
  }

  // field wave(&executor) -> Wave {
  //   match &*self {
  //     Card::BattleCard(card) => card.wave(&executor.context()),
  //     // Card::CharacterCard(card) => card.wave(&executor.context()),
  //   }
  // }

  instance_resolvers: |_| {
    &BattleCard => match *self { Card::BattleCard(ref card) => Some(card), _ => None },
    &CharacterCard => match *self { Card::CharacterCard(ref card) => Some(card), _ => None },
  }
});
