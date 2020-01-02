use crate::database::schema::{cards, waves};
use crate::database::types::{CardCategory, CardRarity};
use crate::graphql_schema::Context;
use diesel::{QueryDsl, RunQueryDsl};
use uuid::Uuid;

#[derive(Identifiable, Queryable, Debug, juniper::GraphQLObject)]
pub struct Wave {
  pub id: Uuid,
  pub tcg_id: String,
  pub name: String,
  pub released: chrono::NaiveDate,
}

#[derive(Identifiable, Queryable, PartialEq, Eq, Debug)]
pub struct Card {
  pub id: Uuid,
  pub tcg_id: String,
  pub rarity: CardRarity,
  pub number: String,
  pub category: CardCategory,
  pub wave_id: Uuid,
}

#[juniper::object(
  Context = Context
)]
impl Card {
  pub fn id(&self) -> Uuid {
    self.id
  }

  pub fn tcg_id(&self) -> &str {
    self.tcg_id.as_str()
  }

  pub fn rarity(&self) -> &CardRarity {
    &self.rarity
  }

  pub fn number(&self) -> &str {
    self.number.as_str()
  }

  pub fn category(&self) -> &CardCategory {
    &self.category
  }

  pub fn wave(&self, context: &Context) -> Wave {
    waves::table
      .inner_join(cards::table)
      .select((waves::id, waves::tcg_id, waves::name, waves::released))
      .first::<Wave>(&context.connection)
      .expect("Error loading posts")
  }
}
