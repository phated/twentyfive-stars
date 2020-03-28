use crate::data::{CharacterMode, CharacterTrait, Faction, ModeType, Node, ID};
use crate::graphql_schema::Context;

pub struct UpgradeMode {
  id: ID,
  title: String,
  stars: i32,
  type_: ModeType,
  faction: Faction,
  traits: Vec<CharacterTrait>,
  // Only available on Upgrade modes
  attack_modifier: Option<i32>,
  defense_modifier: Option<i32>,
}

impl UpgradeMode {
  pub fn new(
    id: ID,
    title: String,
    stars: i32,
    type_: ModeType,
    faction: Faction,
    traits: Vec<CharacterTrait>,
    attack_modifier: Option<i32>,
    defense_modifier: Option<i32>,
  ) -> Self {
    UpgradeMode {
      id,
      title,
      stars,
      type_,
      faction,
      traits,
      attack_modifier,
      defense_modifier,
    }
  }
}

impl UpgradeMode {
  pub fn id(&self) -> ID {
    self.id
  }

  pub fn title(&self) -> &str {
    &self.title
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

  pub fn attack_modifier(&self) -> Option<i32> {
    self.attack_modifier
  }

  pub fn defense_modifier(&self) -> Option<i32> {
    self.defense_modifier
  }
}

juniper::graphql_object!(UpgradeMode: Context | &self | {
  interfaces: [&Node, &CharacterMode]

  field id() -> ID {
    self.id()
  }

  field title() -> &str {
    self.title()
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

  field attack_modifier() -> Option<i32> {
    self.attack_modifier()
  }

  field defense_modifier() -> Option<i32> {
    self.defense_modifier()
  }
});
