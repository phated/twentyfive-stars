use crate::data::{Faction, ModeType, ID};

#[async_graphql::SimpleObject]
#[derive(Clone, Debug)]
pub struct HeadMode {
  pub id: ID,
  pub title: String,
  pub stars: i32,
  pub type_: ModeType,
  pub faction: Faction,
}
