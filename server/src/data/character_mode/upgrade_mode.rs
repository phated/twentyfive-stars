use crate::data::{CharacterTrait, Faction, ModeType};
use async_graphql::ID;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct UpgradeMode {
    pub id: i32,
    pub node_id: Uuid,
    pub title: String,
    pub stars: i32,
    pub type_: ModeType,
    pub faction: Faction,
    pub traits: Vec<CharacterTrait>,
    // Only available on Upgrade modes
    pub attack_modifier: Option<i32>,
    pub defense_modifier: Option<i32>,
}

#[async_graphql::Object]
impl UpgradeMode {
    pub async fn id(&self) -> ID {
        self.node_id.into()
    }

    pub async fn title(&self) -> &str {
        &self.title
    }

    pub async fn stars(&self) -> i32 {
        self.stars
    }

    pub async fn type_(&self) -> ModeType {
        self.type_
    }

    pub async fn faction(&self) -> Faction {
        self.faction
    }

    pub async fn traits(&self) -> &Vec<CharacterTrait> {
        &self.traits
    }

    pub async fn attack_modifier(&self) -> Option<i32> {
        self.attack_modifier
    }

    pub async fn defense_modifier(&self) -> Option<i32> {
        self.defense_modifier
    }
}
