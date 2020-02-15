use crate::data::{BattleIcon, BattleType, Card, CardCategory, CardRarity, Wave};
use crate::database_schema::battle_cards;
use crate::graphql_schema::Context;
use uuid::Uuid;

#[derive(Identifiable, Queryable, PartialEq, Eq, Debug)]
pub struct BattleCard {
  pub id: Uuid,
  pub card_id: Uuid,
  pub title: String,
  pub type_: BattleType,
  pub stars: Option<i32>,
  pub icons: Vec<BattleIcon>,
  pub attack_modifier: Option<i32>,
  pub defense_modifier: Option<i32>,
}

juniper::graphql_object!(BattleCard: Context |&self| {
  interfaces: [&Card]

  field title() -> &str {
    self.title.as_str()
  }

  field stars() -> Option<i32> {
    self.stars
  }

  field icons() -> &Vec<BattleIcon> {
    &self.icons
  }

  field type() -> &BattleType {
    &self.type_
  }

  field attack_modifier() -> Option<i32> {
    self.attack_modifier
  }

  field defense_modifier() -> Option<i32> {
    self.defense_modifier
  }

  // Implemented by the interface
  field id() -> Uuid {
    unimplemented!("`id` field should be implemented by interface")
  }

  field tcg_id() -> &str {
    unimplemented!("`tcg_id` field should be implemented by interface")
  }

  field rarity() -> &CardRarity {
    unimplemented!("`rarity` field should be implemented by interface")
  }

  field number() -> &str {
    unimplemented!("`number` field should be implemented by interface")
  }

  field category() -> &CardCategory {
    unimplemented!("`category` field should be implemented by interface")
  }

  field wave() -> Wave {
    unimplemented!("`wave` field should be implemented by interface")
  }
});
