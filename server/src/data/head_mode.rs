use crate::data::{Faction, ModeType, ID};

#[async_graphql::SimpleObject]
#[derive(Clone, Debug)]
pub struct HeadMode {
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
}
