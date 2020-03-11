use crate::schema::Cursor;

pub trait Edge {
  type EdgeType;

  fn cursor(&self) -> Cursor;

  fn node(&self) -> Self::EdgeType;

  fn has_previous(&self) -> bool;

  fn has_next(&self) -> bool;
}
