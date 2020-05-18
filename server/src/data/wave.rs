use crate::data::ID;
use crate::database_schema::waves;
use chrono::NaiveDate;

#[async_graphql::SimpleObject]
#[derive(Identifiable, Queryable, Debug)]
pub struct Wave {
    pub id: ID,
    pub tcg_id: String,
    pub name: String,
    pub released: NaiveDate,
    // pub sort_order: i32,
}
