use crate::data::{BattleIcon, BattleType, Card, CardCategory, CardRarity, Faction, Wave};
use crate::database_schema::battle_cards;
use crate::graphql_schema::Context;
use diesel::prelude::*;
use juniper::FieldResult;
use uuid::Uuid;

#[derive(Identifiable, Queryable, PartialEq, Eq, Debug)]
#[table_name = "battle_cards"]
pub struct ExtraProps {
  id: Uuid,
  card_id: Uuid,
  title: String,
  type_: BattleType,
  faction: Option<Faction>,
  stars: Option<i32>,
  icons: Vec<BattleIcon>,
  attack_modifier: Option<i32>,
  defense_modifier: Option<i32>,
}

pub struct BattleCard(Card, ExtraProps);

impl BattleCard {
  pub fn new(card: Card, extra: ExtraProps) -> Self {
    BattleCard(card, extra)
  }

  pub fn load_from_card(card: &Card, context: &Context) -> Option<Self> {
    battle_cards::table
      .filter(battle_cards::card_id.eq(card.id))
      .first::<ExtraProps>(&context.connection)
      .ok()
      // TODO: performance of cloning this?
      .map(|extra| BattleCard::new(card.clone(), extra))
  }
}

impl BattleCard {
  pub fn id(&self) -> Uuid {
    match self {
      BattleCard(card, _extra) => card.id(),
    }
  }

  pub fn tcg_id(&self) -> &str {
    match self {
      BattleCard(card, _extra) => card.tcg_id(),
    }
  }

  pub fn rarity(&self) -> &CardRarity {
    match self {
      BattleCard(card, _extra) => card.rarity(),
    }
  }

  pub fn number(&self) -> &str {
    match self {
      BattleCard(card, _extra) => card.number(),
    }
  }

  pub fn category(&self) -> &CardCategory {
    match self {
      BattleCard(card, _extra) => card.category(),
    }
  }

  pub fn wave(&self, context: &Context) -> QueryResult<Wave> {
    match self {
      BattleCard(card, _extra) => card.wave(context),
    }
  }

  pub fn title(&self) -> &str {
    match self {
      BattleCard(_card, extra) => &extra.title,
    }
  }

  pub fn stars(&self) -> Option<i32> {
    match self {
      BattleCard(_card, extra) => extra.stars,
    }
  }

  pub fn icons(&self) -> &Vec<BattleIcon> {
    match self {
      BattleCard(_card, extra) => &extra.icons,
    }
  }

  pub fn type_(&self) -> &BattleType {
    match self {
      BattleCard(_card, extra) => &extra.type_,
    }
  }

  pub fn faction(&self) -> &Option<Faction> {
    match self {
      BattleCard(_card, extra) => &extra.faction,
    }
  }

  pub fn attack_modifier(&self) -> Option<i32> {
    match self {
      BattleCard(_card, extra) => extra.attack_modifier,
    }
  }

  pub fn defense_modifier(&self) -> Option<i32> {
    match self {
      BattleCard(_card, extra) => extra.defense_modifier,
    }
  }
}

juniper::graphql_object!(BattleCard: Context |&self| {
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

  field stars() -> Option<i32> {
    self.stars()
  }

  field icons() -> &Vec<BattleIcon> {
    self.icons()
  }

  field type() -> &BattleType {
    self.type_()
  }

  field faction() -> &Option<Faction> {
    self.faction()
  }

  field attack_modifier() -> Option<i32> {
    self.attack_modifier()
  }

  field defense_modifier() -> Option<i32> {
    self.defense_modifier()
  }
});
