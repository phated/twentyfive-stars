use crate::database::schema::{battle_cards, cards, waves};
use crate::database::types::{BattleIcon, BattleType, CardCategory, CardRarity};
use crate::graphql_schema::Context;
use crate::models::card::Card;
use crate::models::wave::Wave;
use diesel::{QueryDsl, RunQueryDsl};
use uuid::Uuid;

#[derive(Identifiable, Queryable, PartialEq, Eq, Debug)]
pub struct BattleCard {
  pub id: Uuid,
  pub card_id: Uuid,
  pub tcg_id: String,
  pub rarity: CardRarity,
  pub number: String,
  pub category: CardCategory,
  pub wave_id: Uuid,
  // Specific to BattleCard
  pub title: String,
  pub stars: Option<i16>,
  pub icons: Vec<BattleIcon>,
  pub type_: BattleType,
  pub attack_modifier: Option<i16>,
  pub defense_modifier: Option<i16>,
}

impl BattleCard {
  pub fn id(&self) -> Uuid {
    self.id
  }

  pub fn card_id(&self) -> Uuid {
    self.card_id
  }

  pub fn tcg_id(&self) -> &str {
    self.tcg_id.as_str()
  }

  pub fn rarity(&self) -> &CardRarity {
    &self.rarity
  }

  pub fn number(&self) -> &str {
    self.number.as_str()
  }

  pub fn category(&self) -> &CardCategory {
    &self.category
  }

  pub fn wave(&self, context: &Context) -> Wave {
    waves::table
      .inner_join(cards::table)
      .select((waves::id, waves::tcg_id, waves::name, waves::released))
      .first::<Wave>(&context.connection)
      .expect("Error loading wave")
  }

  pub fn title(&self) -> &str {
    self.title.as_str()
  }

  pub fn stars(&self) -> Option<i32> {
    self.stars.map(|stars| stars as i32)
  }

  pub fn icons(&self) -> &Vec<BattleIcon> {
    &self.icons
  }

  pub fn type_(&self) -> &BattleType {
    &self.type_
  }

  pub fn attack_modifier(&self) -> Option<i32> {
    self.attack_modifier.map(|modifier| modifier as i32)
  }

  pub fn defense_modifier(&self) -> Option<i32> {
    self.defense_modifier.map(|modifier| modifier as i32)
  }
}

juniper::graphql_object!(BattleCard: Context |&self| {
  interfaces: [&Card]

  field id() -> Uuid {
    self.id()
  }

  field card_id() -> Uuid {
    self.card_id()
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

  field wave(&executor) -> Wave {
    self.wave(executor.context())
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

  field attack_modifier() -> Option<i32> {
    self.attack_modifier()
  }

  field defense_modifier() -> Option<i32> {
    self.defense_modifier()
  }
});
