use crate::data::{Node, ID};
use crate::database_schema::waves;
use crate::graphql_schema::Context;
use chrono::NaiveDate;
use uuid::Uuid;

#[derive(Identifiable, Queryable, Debug)]
pub struct Wave {
  id: Uuid,
  tcg_id: String,
  name: String,
  released: NaiveDate,
}

impl Wave {
  pub fn id(&self) -> ID {
    ID::WaveID(self.id)
  }

  pub fn tcg_id(&self) -> &str {
    &self.tcg_id
  }

  pub fn name(&self) -> &str {
    &self.name
  }

  pub fn released(&self) -> NaiveDate {
    self.released
  }
}

juniper::graphql_object!(Wave: Context | &self | {
  interfaces: [&Node]

  field id() -> ID {
    self.id()
  }

  field tcg_id() -> &str {
    self.tcg_id()
  }

  field name() -> &str {
    self.name()
  }

  field released() -> NaiveDate {
    self.released()
  }
});
