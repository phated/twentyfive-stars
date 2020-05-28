use crate::data::{CharacterTrait, Faction, ModeType};
use uuid::Uuid;

#[async_graphql::SimpleObject]
#[derive(Clone, Debug)]
pub struct UpgradeMode {
    #[field(skip)]
    pub node_id: Uuid,

    pub id: i32,
    pub title: String,
    pub stars: i32,
    pub type_: ModeType,
    pub faction: Faction,
    pub traits: Vec<CharacterTrait>,
    // Only available on Upgrade modes
    pub attack_modifier: Option<i32>,
    pub defense_modifier: Option<i32>,
}
