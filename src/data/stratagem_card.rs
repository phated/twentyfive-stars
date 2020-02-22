use crate::data::{Card, CardCategory, CardRarity, Faction, Wave};
use crate::database_schema::stratagem_cards;
use crate::graphql_schema::Context;
use diesel::prelude::*;
use juniper::FieldResult;
use uuid::Uuid;

#[derive(Identifiable, Queryable, PartialEq, Eq, Debug)]
#[table_name = "stratagem_cards"]
pub struct ExtraProps {
  id: Uuid,
  card_id: Uuid,
  title: String,
  requirement: String,
  faction: Option<Faction>,
  stars: i32,
}

pub struct StratagemCard(Card, ExtraProps);

impl StratagemCard {
  pub fn new(card: Card, extra: ExtraProps) -> Self {
    StratagemCard(card, extra)
  }

  pub fn load_from_card(card: &Card, context: &Context) -> Option<Self> {
    stratagem_cards::table
      .filter(stratagem_cards::card_id.eq(card.id()))
      .first::<ExtraProps>(&context.connection)
      .ok()
      // TODO: performance of cloning this?
      .map(|extra| StratagemCard::new(card.clone(), extra))
  }
}

impl StratagemCard {
  pub fn id(&self) -> Uuid {
    match self {
      StratagemCard(card, _extra) => card.id(),
    }
  }

  pub fn tcg_id(&self) -> &str {
    match self {
      StratagemCard(card, _extra) => card.tcg_id(),
    }
  }

  pub fn rarity(&self) -> &CardRarity {
    match self {
      StratagemCard(card, _extra) => card.rarity(),
    }
  }

  pub fn number(&self) -> &str {
    match self {
      StratagemCard(card, _extra) => card.number(),
    }
  }

  pub fn category(&self) -> &CardCategory {
    match self {
      StratagemCard(card, _extra) => card.category(),
    }
  }

  pub fn wave(&self, context: &Context) -> QueryResult<Wave> {
    match self {
      StratagemCard(card, _extra) => card.wave(context),
    }
  }

  pub fn title(&self) -> &str {
    match self {
      StratagemCard(_card, extra) => &extra.title,
    }
  }

  pub fn requirement(&self) -> &str {
    match self {
      StratagemCard(_card, extra) => &extra.requirement,
    }
  }

  pub fn stars(&self) -> i32 {
    match self {
      StratagemCard(_card, extra) => extra.stars,
    }
  }

  pub fn faction(&self) -> &Option<Faction> {
    match self {
      StratagemCard(_card, extra) => &extra.faction,
    }
  }
}

juniper::graphql_object!(StratagemCard: Context |&self| {
  interfaces: [&Card]

  field id() -> Uuid {
    self.id()
  }

  field tcg_id() -> &str {
    self.tcg_id()
  }

  field rarity() -> &CardRarity {
    self.rarity()
  }

  field number() -> &str {
    self.number()
  }

  field category() -> &CardCategory {
    self.category()
  }

  field wave(&executor) -> FieldResult<Wave> {
    let context = executor.context();
    // TODO: weird conversion between result types
    let wave = self.wave(context)?;
    Ok(wave)
  }

  field title() -> &str {
    self.title()
  }

  field requirement() -> &str {
    self.requirement()
  }

  field stars() -> i32 {
    self.stars()
  }

  field faction() -> &Option<Faction> {
    self.faction()
  }
});
