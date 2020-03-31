use crate::graphql_schema::Context;
use crate::schema::{Cursor, Edge};

pub struct PageInfo {
  has_previous_page: bool,
  has_next_page: bool,
  start_cursor: Cursor,
  end_cursor: Cursor,
}

impl PageInfo {
  pub fn from_edges(edges: &Vec<impl Edge>) -> Self {
    let (has_previous_page, start_cursor) = match edges.first() {
      Some(edge) => {
        let start_cursor = edge.cursor();
        let has_previous = edge.has_previous();
        (has_previous, start_cursor)
      }
      None => (false, Cursor::empty()),
    };

    let (has_next_page, end_cursor) = match edges.last() {
      Some(edge) => {
        let end_cursor = edge.cursor();
        let has_next = edge.has_next();
        (has_next, end_cursor)
      }
      None => (false, Cursor::empty()),
    };

    PageInfo {
      has_previous_page,
      has_next_page,
      start_cursor,
      end_cursor,
    }
  }
}

impl PageInfo {
  pub fn has_previous_page(&self) -> bool {
    self.has_previous_page
  }

  pub fn has_next_page(&self) -> bool {
    self.has_next_page
  }

  pub fn start_cursor(&self) -> Cursor {
    self.start_cursor
  }

  pub fn end_cursor(&self) -> Cursor {
    self.end_cursor
  }
}

juniper::graphql_object!(PageInfo: Context | &self | {
  field has_previous_page() -> bool {
    self.has_previous_page()
  }

  field has_next_page() -> bool {
    self.has_next_page()
  }

  field start_cursor() -> Cursor {
    self.start_cursor()
  }

  field end_cursor() -> Cursor {
    self.end_cursor()
  }
});
