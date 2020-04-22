use crate::data::ID;
use crate::database_schema::waves;
use crate::schema::Date;

#[async_graphql::SimpleObject]
#[derive(Identifiable, Queryable, Debug)]
pub struct Wave {
  #[field]
  pub id: ID,
  #[field]
  pub tcg_id: String,
  #[field]
  pub name: String,
  #[field]
  pub released: Date,
  pub sort_order: i32,
}
