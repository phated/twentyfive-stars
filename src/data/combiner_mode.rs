use crate::data::{CharacterMode, CharacterTrait, Faction, ModeType, Node, ID};
use crate::graphql_schema::Context;
use uuid::Uuid;

pub struct CombinerMode {
  id: Uuid,
  title: String,
  subtitle: String,
  stars: i32,
  type_: ModeType,
  faction: Faction,
  traits: Vec<CharacterTrait>,
  // Not available on Head or Upgrade modes
  health: i32,
  attack: i32,
  defense: i32,
}

impl CombinerMode {
  pub fn new(
    id: Uuid,
    title: String,
    subtitle: String,
    stars: i32,
    type_: ModeType,
    faction: Faction,
    traits: Vec<CharacterTrait>,
    health: i32,
    attack: i32,
    defense: i32,
  ) -> Self {
    CombinerMode {
      id,
      title,
      subtitle,
      stars,
      type_,
      faction,
      traits,
      health,
      attack,
      defense,
    }
  }
}

impl CombinerMode {
  pub fn id(&self) -> ID {
    ID::CharacterModeID(self.id)
  }

  pub fn title(&self) -> &str {
    &self.title
  }

  pub fn subtitle(&self) -> &str {
    &self.subtitle
  }

  pub fn stars(&self) -> i32 {
    self.stars
  }

  pub fn type_(&self) -> &ModeType {
    &self.type_
  }

  pub fn faction(&self) -> &Faction {
    &self.faction
  }

  pub fn traits(&self) -> &Vec<CharacterTrait> {
    &self.traits
  }

  pub fn health(&self) -> i32 {
    self.health
  }

  pub fn attack(&self) -> i32 {
    self.attack
  }

  pub fn defense(&self) -> i32 {
    self.defense
  }
}

juniper::graphql_object!(CombinerMode: Context | &self | {
  interfaces: [&Node, &CharacterMode]

  field id() -> ID {
    self.id()
  }

  field title() -> &str {
    self.title()
  }

  field subtitle() -> &str {
    self.subtitle()
  }

  field stars() -> i32 {
    self.stars()
  }

  field type_() -> &ModeType {
    self.type_()
  }

  field faction() -> &Faction {
    self.faction()
  }

  field traits() -> &Vec<CharacterTrait> {
    self.traits()
  }

  field health() -> i32 {
    self.health()
  }

  field attack() -> i32 {
    self.attack()
  }

  field defense() -> i32 {
    self.defense()
  }
});
