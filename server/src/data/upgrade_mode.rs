use crate::data::{CharacterTrait, Faction, ModeType, ID};

#[async_graphql::SimpleObject]
#[derive(Clone, Debug)]
pub struct UpgradeMode {
  #[field]
  pub id: ID,
  #[field]
  pub title: String,
  #[field]
  pub stars: i32,
  #[field]
  pub type_: ModeType,
  #[field]
  pub faction: Faction,
  #[field]
  pub traits: Vec<CharacterTrait>,
  // Only available on Upgrade modes
  #[field]
  pub attack_modifier: Option<i32>,
  #[field]
  pub defense_modifier: Option<i32>,
}
