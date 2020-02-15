use crate::data::{CharacterMode, CharacterTrait, ModeType};
use crate::graphql_schema::Context;
use uuid::Uuid;

pub struct BotMode {
  pub id: Uuid,
  pub title: String,
  pub subtitle: String,
  pub stars: i32,
  pub type_: ModeType,
  pub traits: Vec<CharacterTrait>,
  // Not available on Head or Upgrade modes
  pub health: i32,
  pub attack: i32,
  pub defense: i32,
}

juniper::graphql_object!(BotMode: Context | &self | {
  interfaces: [&CharacterMode]

  field id() -> Uuid {
    self.id
  }

  field title() -> &str {
    &self.title
  }

  field subtitle() -> &str {
    &self.subtitle
  }

  field stars() -> i32 {
    self.stars
  }

  field type() -> &ModeType {
    &self.type_
  }

  field traits() -> &Vec<CharacterTrait> {
    &self.traits
  }

  field health() -> i32 {
    self.health
  }

  field attack() -> i32 {
    self.attack
  }

  field defense() -> i32 {
    self.defense
  }
});
