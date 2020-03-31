use crate::data::{Card, CharacterMode, Wave, ID};
use crate::graphql_schema::Context;

pub enum Node {
  Card(Card),
  Wave(Wave),
  CharacterMode(CharacterMode),
}

impl From<Card> for Node {
  fn from(card: Card) -> Node {
    Node::Card(card)
  }
}
impl From<Wave> for Node {
  fn from(wave: Wave) -> Node {
    Node::Wave(wave)
  }
}

juniper::graphql_interface!(Node: Context |&self| {
  field id() -> ID {
    match self {
      Node::Card(card) => card.id(),
      Node::Wave(wave) => wave.id(),
      Node::CharacterMode(mode) => mode.id()
    }
  }

  instance_resolvers: |_| {
    &Card => match self { Node::Card(card) => Some(card), _ => None },
    &Wave => match self { Node::Wave(wave) => Some(wave), _ => None },
    &CharacterMode => match self { Node::CharacterMode(mode) => Some(mode), _ => None },
  }
});
