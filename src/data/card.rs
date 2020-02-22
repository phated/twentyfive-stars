use crate::data::{BattleCard, CardCategory, CardRarity, CharacterCard, Wave};
use crate::database_schema::{cards, waves};
use crate::graphql_schema::Context;
use diesel::prelude::*;
use juniper::FieldResult;
use uuid::Uuid;

#[derive(Identifiable, Queryable, Clone, PartialEq, Eq, Debug)]
pub struct Card {
  pub id: Uuid,
  pub tcg_id: String,
  pub rarity: CardRarity,
  pub number: String,
  pub category: CardCategory,
  pub wave_id: Uuid,
}

impl Card {
  pub fn id(&self) -> Uuid {
    self.id
  }

  pub fn tcg_id(&self) -> &str {
    &self.tcg_id
  }

  pub fn rarity(&self) -> &CardRarity {
    &self.rarity
  }

  pub fn number(&self) -> &str {
    &self.number
  }

  pub fn category(&self) -> &CardCategory {
    &self.category
  }

  pub fn wave(&self, context: &Context) -> QueryResult<Wave> {
    waves::table
      .filter(waves::id.eq(self.wave_id))
      .first::<Wave>(&context.connection)
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

  field wave(&executor) -> FieldResult<Wave> {
    let context = executor.context();
    // TODO: weird conversion between result types
    let wave = self.wave(context)?;
    Ok(wave)
  }

  instance_resolvers: |&context| {
    BattleCard => {
      match self.category {
        CardCategory::Battle => {
          let card = self;
          BattleCard::load_from_card(card, context)
        }
        _ => None
      }
    },
    CharacterCard => {
      match self.category {
        CardCategory::Character => {
          let card = self;
          CharacterCard::load_from_card(card, context)
        },
        _ => None
      }
    }
  }
});
