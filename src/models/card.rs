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

impl Card {
  pub fn id(&self) -> Uuid {
    match *self {
      Card::BattleCard(BattleCard { id, .. }) => id,
    }
  }

  pub fn tcg_id(&self) -> &str {
    match *self {
      Card::BattleCard(BattleCard { ref tcg_id, .. }) => tcg_id.as_str(),
    }
  }

  pub fn rarity(&self) -> &CardRarity {
    match *self {
      Card::BattleCard(BattleCard { ref rarity, .. }) => rarity,
    }
  }

  pub fn number(&self) -> &str {
    match *self {
      Card::BattleCard(BattleCard { ref number, .. }) => number.as_str(),
    }
  }

  pub fn category(&self) -> &CardCategory {
    match *self {
      Card::BattleCard(BattleCard { ref category, .. }) => category,
    }
  }

  pub fn wave(&self, context: &Context) -> Wave {
    match &*self {
      Card::BattleCard(card) => card.wave(context),
    }
  }
}

juniper::graphql_interface!(Card: Context |&self| {
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

  field wave(&executor) -> Wave {
    self.wave(&executor.context())
  }

  instance_resolvers: |_| {
    &BattleCard => match *self { Card::BattleCard(ref card) => Some(card), _ => None },
      // BattleCard => {
      //   let results = battle_cards::table
      //   .inner_join(cards::table)
      //   .select((battle_cards::id, battle_cards::card_id, cards::tcg_id, cards::rarity, cards::number, cards::category, cards::wave_id))
      //   .first::<BattleCard>(&context.connection);
      //   match results {
      //     Ok(res) => Some(res),
      //     Err(_) => None
      //   }
      // }
      // &Droid => context.droids.get(&self.id),
  }
});
