use crate::data::{BattleIcon, BattleType, Card, CardCategory, CardRarity, Faction, Wave, ID};
use crate::database::{Database, Error};
use crate::database_schema::battle_cards;
use crate::schema::Cursor;
use async_graphql::{Context, FieldResult};
use diesel::prelude::*;

#[derive(Identifiable, Queryable, Clone, PartialEq, Eq, Debug)]
#[table_name = "battle_cards"]
pub struct ExtraProps {
  id: ID,
  card_id: ID,
  title: String,
  type_: BattleType,
  faction: Option<Faction>,
  stars: Option<i32>,
  icons: Vec<BattleIcon>,
  attack_modifier: Option<i32>,
  defense_modifier: Option<i32>,
  sort_order: i32,
}

#[derive(Clone, Debug)]
pub struct BattleCard(Card, ExtraProps);

impl BattleCard {
  pub fn new(card: Card, extra: ExtraProps) -> Self {
    BattleCard(card, extra)
  }

  // pub fn load_from_card(card: Card, pool: &ConnPool) -> Result<BattleCard, Error> {
  //   let conn = pool.get().unwrap();

  //   battle_cards::table
  //     .filter(battle_cards::card_id.eq(card.id))
  //     .first::<ExtraProps>(&conn)
  //     // TODO: performance of cloning this?
  //     .map(|extra| BattleCard::new(card.clone(), extra))
  // }
}

#[async_graphql::Object]
impl BattleCard {
  #[field(skip)]
  pub fn cursor(&self) -> Cursor {
    Cursor::new(self.0.id)
  }

  pub async fn id(&self) -> ID {
    self.0.id
  }

  pub async fn tcg_id(&self) -> &str {
    &self.0.tcg_id
  }

  pub async fn rarity(&self) -> CardRarity {
    self.0.rarity
  }

  pub async fn number(&self) -> &str {
    &self.0.number
  }

  pub async fn category(&self) -> CardCategory {
    self.0.category
  }

  pub async fn wave(&self, ctx: &Context<'_>) -> FieldResult<Wave> {
    let db = ctx.data::<Database>();
    let wave = db.get_wave(self.0.wave_id)?;
    Ok(wave)
  }

  pub async fn title(&self) -> &str {
    &self.1.title
  }

  pub async fn stars(&self) -> Option<i32> {
    self.1.stars
  }

  pub async fn icons(&self) -> &Vec<BattleIcon> {
    &self.1.icons
  }

  pub async fn type_(&self) -> BattleType {
    self.1.type_
  }

  pub async fn faction(&self) -> Option<Faction> {
    self.1.faction
  }

  pub async fn attack_modifier(&self) -> Option<i32> {
    self.1.attack_modifier
  }

  pub async fn defense_modifier(&self) -> Option<i32> {
    self.1.defense_modifier
  }
}
