use crate::data::{Faction, ModeType};
use uuid::Uuid;

#[async_graphql::SimpleObject]
#[derive(Clone, Debug)]
pub struct HeadMode {
    #[field(skip)]
    pub node_id: Uuid,

    pub id: i32,
    pub title: String,
    pub stars: i32,
    pub type_: ModeType,
    pub faction: Faction,
}
