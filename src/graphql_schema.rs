use crate::data::{BattleCard, Card, CharacterCard};
use crate::database;
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

  fn all_cards(context: &Context) -> FieldResult<Vec<Card>> {
    let cards = database::get_cards(&context.connection)?;
    // TODO: weird conversion between result types
    Ok(cards)
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
