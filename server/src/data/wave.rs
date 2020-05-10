use crate::data::ID;
use crate::database_schema::waves;
use crate::schema::Date;
use chrono::NaiveDate;
use uuid::Uuid;

#[async_graphql::SimpleObject]
#[derive(Debug, Clone)]
pub struct Wave {
  pub id: ID,
  pub tcg_id: String,
  pub name: String,
  pub released: Date,
  // pub sort_order: i32,
}
