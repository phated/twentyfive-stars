use async_graphql::{GQLInputObject, ID};
use chrono::NaiveDate;
use uuid::Uuid;

#[derive(Debug, Clone, GQLInputObject)]
pub struct WaveInput {
    pub tcg_id: String,
    pub name: String,
    pub released: NaiveDate,
}

#[derive(Debug, Clone)]
pub struct Wave {
    pub id: i32,
    pub node_id: Uuid,
    pub tcg_id: String,
    pub name: String,
    pub released: NaiveDate,
}

#[async_graphql::Object]
impl Wave {
    pub async fn id(&self) -> ID {
        self.node_id.into()
    }

    pub async fn tcg_id(&self) -> &str {
        &self.tcg_id
    }

    pub async fn name(&self) -> &str {
        &self.name
    }

    pub async fn released(&self) -> NaiveDate {
        self.released
    }
}
