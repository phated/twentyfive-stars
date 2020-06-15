use crate::data::{CharacterTrait, Faction, ModeType};
use async_graphql::ID;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct CombinerMode {
    pub id: i32,
    pub node_id: Uuid,
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

#[async_graphql::Object]
impl CombinerMode {
    pub async fn id(&self) -> ID {
        self.node_id.into()
    }

    pub async fn title(&self) -> &str {
        &self.title
    }

    pub async fn subtitle(&self) -> &str {
        &self.subtitle
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

    pub async fn health(&self) -> i32 {
        self.health
    }

    pub async fn attack(&self) -> i32 {
        self.attack
    }

    pub async fn defense(&self) -> i32 {
        self.defense
    }
}
