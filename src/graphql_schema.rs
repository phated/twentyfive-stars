use juniper::{FieldResult};

// use self::models::*;
use diesel::prelude::*;
use crate::database::{get_cards};
use crate::database::models::{Card};

// #[derive(juniper::GraphQLEnum)]
// pub enum CardType {
//     Character,
//     Action,
//     Upgrade,
// }

// #[derive(juniper::GraphQLObject)]
// /// A card in the Transformers TCG.
// pub struct Card {
//   id: i32,
//   title: String,
//   type_: CardType,
// }

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
      // Ok(Card {
      //   id: 1,
      //   title: String::from("Optimus"),
      //   type_: CardType::Character
      // })
    let results = get_cards(&context.connection);

    Ok(results)
  }
}

// #[derive(juniper::GraphQLInputObject)]
// struct InputCard {
//     title: String,
//     type_: CardType,
// }

pub struct Mutation;
#[juniper::object(
  Context = Context,
)]
impl Mutation {
  fn addCard(context: &Context) -> FieldResult<String> {
    Ok(String::from("hello"))
  }
}

pub type Schema = juniper::RootNode<'static, Query, Mutation>;
