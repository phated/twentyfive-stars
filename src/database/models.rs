use crate::database::schema::{cards, waves};
use crate::graphql_schema::Context;
use diesel::{QueryDsl, RunQueryDsl};
use uuid::Uuid;

#[derive(Identifiable, Queryable, Debug, juniper::GraphQLObject)]
pub struct Wave {
  pub id: String,
  pub name: String,
  pub released: chrono::NaiveDate,
}

#[derive(Identifiable, Queryable, PartialEq, Eq, Debug)]
pub struct Card {
  pub id: Uuid,
  pub card_number: String,
  pub title: String,
  pub subtitle: Option<String>,
  pub wave_id: String,
}

#[juniper::object(
  Context = Context
)]
impl Card {
  pub fn id(&self) -> Uuid {
    self.id
  }

  pub fn card_number(&self) -> &str {
    self.card_number.as_str()
  }

  pub fn title(&self) -> &str {
    self.title.as_str()
  }

  pub fn subtitle(&self) -> Option<&str> {
    self.subtitle.as_ref().map(|subtitle| subtitle.as_str())
  }

  pub fn wave(&self, context: &Context) -> Wave {
    waves::table
      .inner_join(cards::table)
      .select((waves::id, waves::name, waves::released))
      .first::<Wave>(&context.connection)
      .expect("Error loading posts")
  }
}
