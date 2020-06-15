use crate::data::{Faction, ModeType};
use async_graphql::ID;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct HeadMode {
    pub id: i32,
    pub node_id: Uuid,
    pub title: String,
    pub stars: i32,
    pub type_: ModeType,
    pub faction: Faction,
}

#[async_graphql::Object]
impl HeadMode {
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
}
