use crate::data::{CharacterTrait, Faction, ModeType, ID};

#[async_graphql::SimpleObject]
#[derive(Clone, Debug)]
pub struct AltMode {
  pub id: ID,
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
