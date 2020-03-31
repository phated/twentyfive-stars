use crate::data::{CharacterMode, Faction, ModeType, Node, ID};
use crate::graphql_schema::Context;

pub struct HeadMode {
  id: ID,
  title: String,
  stars: i32,
  type_: ModeType,
  faction: Faction,
}

impl HeadMode {
  pub fn new(id: ID, title: String, stars: i32, type_: ModeType, faction: Faction) -> Self {
    HeadMode {
      id,
      title,
      stars,
      type_,
      faction,
    }
  }
}

impl HeadMode {
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
}

juniper::graphql_object!(HeadMode: Context | &self | {
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
});
