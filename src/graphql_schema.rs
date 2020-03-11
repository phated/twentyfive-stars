use crate::data::{BattleCard, CharacterCard, Node, StratagemCard, ID};
use crate::database;
use crate::pagination::Pagination;
use crate::schema::{CardConnection, Cursor};
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
    // TODO: parse cargo,toml and expose that?
    "1.0"
  }

  fn node(context: &Context, id: ID) -> FieldResult<Option<Node>> {
    Ok(None)
  }

  fn all_cards(
    context: &Context,
    first: Option<i32>,
    after: Option<Cursor>,
    last: Option<i32>,
    before: Option<Cursor>,
  ) -> FieldResult<CardConnection> {
    let pagination = Pagination::new(first, last, before, after);
    let cards = database::get_cards(&context.connection, pagination)?;
    let connection = CardConnection::new(cards);
    Ok(connection)
  }

  fn all_character_cards(context: &Context) -> FieldResult<Vec<CharacterCard>> {
    let cards = database::get_character_cards(&context.connection)?
      .iter()
      .filter_map(|card| CharacterCard::load_from_card(card, context))
      .collect();
    // TODO: weird conversion between result types
    Ok(cards)
  }

  fn all_battle_cards(context: &Context) -> FieldResult<Vec<BattleCard>> {
    let cards = database::get_battle_cards(&context.connection)?
      .iter()
      .filter_map(|card| BattleCard::load_from_card(card, context))
      .collect();
    // TODO: weird conversion between result types
    Ok(cards)
  }

  fn all_stratagem_cards(context: &Context) -> FieldResult<Vec<StratagemCard>> {
    let cards = database::get_stratagem_cards(&context.connection)?
      .iter()
      .filter_map(|card| StratagemCard::load_from_card(card, context))
      .collect();
    // TODO: weird conversion between result types
    Ok(cards)
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
