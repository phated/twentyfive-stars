use crate::data::{CharacterTrait, Faction, ModeType, ID};

#[async_graphql::SimpleObject]
#[derive(Clone, Debug)]
pub struct CombinerMode {
  #[field]
  pub id: ID,
  #[field]
  pub title: String,
  #[field]
  pub subtitle: String,
  #[field]
  pub stars: i32,
  #[field]
  pub type_: ModeType,
  #[field]
  pub faction: Faction,
  #[field]
  pub traits: Vec<CharacterTrait>,
  // Not available on Head or Upgrade modes
  #[field]
  pub health: i32,
  #[field]
  pub attack: i32,
  #[field]
  pub defense: i32,
}
