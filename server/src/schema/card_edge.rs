use crate::data::Card;
use crate::graphql_schema::Context;
use crate::schema::{Cursor, Edge};

#[derive(Clone)]
pub struct CardEdge {
  cursor: Cursor,
  node: Card,
}

impl Edge for CardEdge {
  type EdgeType = Card;

  fn cursor(&self) -> Cursor {
    self.cursor
  }

  fn node(&self) -> Self::EdgeType {
    self.node.clone()
  }

  fn has_previous(&self) -> bool {
    self.node.has_previous
  }

  fn has_next(&self) -> bool {
    self.node.has_next
  }
}

impl CardEdge {
  pub fn new(card: Card) -> Self {
    CardEdge {
      cursor: Cursor::new(card.id()),
      node: card,
    }
  }
}

juniper::graphql_object!(CardEdge: Context | &self | {
  field cursor() -> Cursor {
    self.cursor()
  }

  field node() -> Card {
    self.node()
  }
});
