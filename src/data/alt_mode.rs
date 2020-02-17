use crate::data::{CharacterMode, CharacterTrait, Faction, ModeType};
use crate::graphql_schema::Context;
use uuid::Uuid;

pub struct AltMode {
  pub id: Uuid,
  pub title: String,
  pub subtitle: String,
  pub stars: i32,
  pub type_: ModeType,
  pub faction: Faction,
  pub traits: Vec<CharacterTrait>,
  // Not available on Head or Upgrade modes
  pub health: i32,
  pub attack: i32,
  pub defense: i32,
}

juniper::graphql_object!(AltMode: Context | &self | {
  interfaces: [&CharacterMode]

  field subtitle() -> &str {
    &self.subtitle
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

  // Implemented by the interface
  field id() -> Uuid {
    unimplemented!("`id` field should be implemented by interface")
  }

  field title() -> &str {
    unimplemented!("`title` field should be implemented by interface")
  }

  field stars() -> i32 {
    unimplemented!("`stars` field should be implemented by interface")
  }

  field type_() -> &ModeType {
    unimplemented!("`type_` field should be implemented by interface")
  }

  field faction() -> &Faction {
    unimplemented!("`faction` field should be implemented by interface")
  }

  field traits() -> &Vec<CharacterTrait> {
    unimplemented!("`type_` field should be implemented by interface")
  }
});
