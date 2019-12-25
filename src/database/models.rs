use crate::database::schema::{cards, waves};
use crate::diesel::{QueryDsl, RunQueryDsl};
use crate::graphql_schema::Context;

#[derive(Identifiable, Queryable, Debug, juniper::GraphQLObject)]
pub struct Wave {
  pub id: String,
  pub name: String,
  pub released: chrono::NaiveDate,
}

#[derive(Identifiable, Queryable, Debug)]
pub struct Card {
  pub id: String,
  pub wave_id: String,
  pub title: String,
}

#[juniper::object(
  Context = Context
)]
impl Card {
  pub fn id(&self) -> String {
    String::from(&self.id)
  }

  pub fn wave(&self, context: &Context) -> Wave {
    waves::table
      .inner_join(cards::table)
      .select((waves::id, waves::name, waves::released))
      .first::<Wave>(&context.connection)
      .expect("Error loading posts")
  }

  pub fn title(&self) -> String {
    String::from(&self.title)
  }
}
