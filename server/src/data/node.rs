use crate::data::{BattleCard, CharacterCard, StratagemCard, Wave, ID};

#[async_graphql::Interface(field(name = "id", type = "ID"))]
pub struct Node(
  BattleCard,
  CharacterCard,
  StratagemCard,
  // TODO: get methods supported on interfaces
  // CharacterMode,
  Wave,
);
