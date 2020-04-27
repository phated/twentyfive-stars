use crate::data::{CharacterTrait, Faction, ModeType, ID};

#[async_graphql::SimpleObject]
#[derive(Clone, Debug)]
pub struct UpgradeMode {
  pub id: ID,
  pub title: String,
  pub stars: i32,
  pub type_: ModeType,
  pub faction: Faction,
  pub traits: Vec<CharacterTrait>,
  // Only available on Upgrade modes
  pub attack_modifier: Option<i32>,
  pub defense_modifier: Option<i32>,
}
