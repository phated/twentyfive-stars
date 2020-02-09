use crate::database::types::{CardCategory, CardRarity};
use crate::graphql_schema::Context;
use crate::models::battle_card::BattleCard;
use crate::models::wave::Wave;
use uuid::Uuid;

pub enum Card {
  BattleCard(BattleCard),
}

impl From<BattleCard> for Card {
  fn from(card: BattleCard) -> Self {
    Card::BattleCard(card)
  }
}

juniper::graphql_interface!(Card: Context |&self| {
  field id() -> Uuid {
    match *self {
      Card::BattleCard(BattleCard { id, .. }) => id,
    }
  }

  field tcg_id() -> &str {
    match *self {
      Card::BattleCard(BattleCard { ref tcg_id, .. }) => tcg_id.as_str(),
    }
  }

  field rarity() -> &CardRarity {
    match *self {
      Card::BattleCard(BattleCard { ref rarity, .. }) => rarity,
    }
  }

  field number() -> &str {
    match *self {
      Card::BattleCard(BattleCard { ref number, .. }) => number.as_str(),
    }
  }

  field category() -> &CardCategory {
    match *self {
      Card::BattleCard(BattleCard { ref category, .. }) => category,
    }
  }

  field wave(&executor) -> Wave {
    match &*self {
      Card::BattleCard(card) => card.wave(&executor.context()),
    }
  }

  instance_resolvers: |_| {
    &BattleCard => match *self { Card::BattleCard(ref card) => Some(card), _ => None },
  }
});
