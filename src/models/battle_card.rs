use crate::database::schema::{battle_cards, cards, waves};
use crate::database::types::{CardCategory, CardRarity};
use crate::graphql_schema::Context;
use crate::models::card::Card;
use crate::models::wave::Wave;
use diesel::{QueryDsl, RunQueryDsl};
use uuid::Uuid;

#[derive(Identifiable, Queryable, PartialEq, Eq, Debug)]
pub struct BattleCard {
  pub id: Uuid,
  pub card_id: Uuid,
  pub tcg_id: String,
  pub rarity: CardRarity,
  pub number: String,
  pub category: CardCategory,
  pub wave_id: Uuid,
}

impl BattleCard {
  pub fn id(&self) -> Uuid {
    self.id
  }

  pub fn card_id(&self) -> Uuid {
    self.card_id
  }

  pub fn tcg_id(&self) -> &str {
    self.tcg_id.as_str()
  }

  pub fn rarity(&self) -> &CardRarity {
    &self.rarity
  }

  pub fn number(&self) -> &str {
    self.number.as_str()
  }

  pub fn category(&self) -> &CardCategory {
    &self.category
  }

  pub fn wave(&self, context: &Context) -> Wave {
    waves::table
      .inner_join(cards::table)
      .select((waves::id, waves::tcg_id, waves::name, waves::released))
      .first::<Wave>(&context.connection)
      .expect("Error loading wave")
  }
}

juniper::graphql_object!(BattleCard: Context |&self| {
  interfaces: [&Card]

  field id() -> Uuid {
    self.id()
  }

  field card_id() -> Uuid {
    self.card_id()
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
    self.wave(executor.context())
  }
});
