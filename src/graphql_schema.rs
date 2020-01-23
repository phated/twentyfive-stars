use crate::database::get_cards;
use crate::models::Card;
use diesel::prelude::PgConnection;
use juniper::FieldResult;

pub struct Context {
  pub connection: PgConnection,
}
impl juniper::Context for Context {}

pub struct Query;
#[juniper::object(
  Context = Context,
)]
impl Query {
  fn apiVersion() -> &str {
    "1.0"
  }

  fn cards(context: &Context) -> FieldResult<Vec<Card>> {
    let results = get_cards(&context.connection);

    Ok(results)
  }
}

pub struct Mutation;
#[juniper::object(
  Context = Context,
)]
impl Mutation {
  fn addCard(context: &Context) -> FieldResult<String> {
    Ok(String::from("hello"))
  }
}
