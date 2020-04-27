use crate::data::ID;
use crate::database_schema::waves;
use crate::schema::Date;

#[async_graphql::SimpleObject]
#[derive(Identifiable, Queryable, Debug)]
pub struct Wave {
  pub id: ID,
  pub tcg_id: String,
  pub name: String,
  pub released: Date,
  pub sort_order: i32,
}
