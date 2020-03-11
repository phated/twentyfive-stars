use crate::data::Card;
use crate::graphql_schema::Context;
use crate::schema::{CardEdge, PageInfo};

pub struct CardConnection {
  page_info: PageInfo,
  edges: Vec<CardEdge>,
}

impl CardConnection {
  pub fn new(cards: Vec<Card>) -> Self {
    let edges = cards
      .iter()
      .map(|card| CardEdge::new(card.clone()))
      .collect();
    let page_info = PageInfo::from_edges(&edges);
    CardConnection { page_info, edges }
  }
}

impl CardConnection {
  pub fn page_info(&self) -> &PageInfo {
    &self.page_info
  }

  pub fn edges(&self) -> &Vec<CardEdge> {
    &self.edges
  }
}

juniper::graphql_object!(CardConnection: Context | &self | {
  field page_info() -> &PageInfo {
    self.page_info()
  }

  field edges() -> &Vec<CardEdge> {
    self.edges()
  }
});
