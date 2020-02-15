use crate::data::{BattleCard, CardCategory, CardRarity, CharacterCard, CharacterMode, Wave};
use crate::database_schema::{battle_cards, cards, character_modes, waves};
use crate::graphql_schema::Context;
use diesel::prelude::*;
use juniper::FieldResult;
use uuid::Uuid;

#[derive(Identifiable, Queryable, PartialEq, Eq, Debug)]
pub struct Card {
  pub id: Uuid,
  pub tcg_id: String,
  pub rarity: CardRarity,
  pub number: String,
  pub category: CardCategory,
  pub wave_id: Uuid,
}

juniper::graphql_interface!(Card: Context |&self| {
  field id() -> Uuid {
    self.id
  }

  field tcg_id() -> &str {
    &self.tcg_id
  }

  field rarity() -> &CardRarity {
    &self.rarity
  }

  field number() -> &str {
    &self.number
  }

  field category() -> &CardCategory {
    &self.category
  }

  field wave(&executor) -> FieldResult<Wave> {
    let context = executor.context();
    let wave = waves::table
      .filter(waves::id.eq(self.wave_id))
      .first::<Wave>(&context.connection)?;
    Ok(wave)
  }

  instance_resolvers: |&context| {
    BattleCard => {
      match self.category {
        CardCategory::Battle => battle_cards::table
          .filter(battle_cards::card_id.eq(self.id))
          .first::<BattleCard>(&context.connection)
          .ok(),
        _ => None
      }
    },
    CharacterCard => {
      match self.category {
        CardCategory::Character => {
          let maybe_modes = character_modes::table
            .filter(character_modes::card_id.eq(self.id))
            .load::<CharacterMode>(&context.connection)
            .ok();
          maybe_modes.map(|modes| CharacterCard { modes })
        },
        _ => None
      }
    }
  }
});
