use crate::data::{BattleIcon, BattleType, CardCategory, CardRarity, Faction, Wave, ID};
use crate::database::{Database, Error};
use crate::database_schema::battle_cards;
use async_graphql::{Context, Cursor, FieldResult};
use diesel::prelude::*;

#[derive(Identifiable, Queryable, Clone, PartialEq, Eq, Debug)]
#[table_name = "battle_cards"]
pub struct BattleCard {
  // Base Card
  pub id: ID,
  pub tcg_id: String,
  pub rarity: CardRarity,
  pub number: String,
  pub category: CardCategory,
  pub wave_id: ID,
  pub sort_order: i32,
  pub has_previous: bool,
  pub has_next: bool,
  // Battle Card
  pub title: String,
  pub type_: BattleType,
  pub faction: Option<Faction>,
  pub stars: Option<i32>,
  pub icons: Vec<BattleIcon>,
  pub attack_modifier: Option<i32>,
  pub defense_modifier: Option<i32>,
}

// #[derive(Clone, Debug)]
// pub struct BattleCard(Card, ExtraProps);

impl BattleCard {
  // pub fn new(card: Card, extra: ExtraProps) -> Self {
  //   BattleCard(card, extra)
  // }

  // pub fn load_from_card(card: Card, pool: &ConnPool) -> Result<BattleCard, Error> {
  //   let conn = pool.get().unwrap();

  //   battle_cards::table
  //     .filter(battle_cards::card_id.eq(card.id))
  //     .first::<ExtraProps>(&conn)
  //     // TODO: performance of cloning this?
  //     .map(|extra| BattleCard::new(card.clone(), extra))
  // }
}

impl Into<Cursor> for BattleCard {
  fn into(self) -> Cursor {
    self.id.into()
  }
}

#[async_graphql::Object]
impl BattleCard {
  pub async fn id(&self) -> ID {
    self.id
  }

  pub async fn tcg_id(&self) -> &str {
    &self.tcg_id
  }

  pub async fn rarity(&self) -> CardRarity {
    self.rarity
  }

  pub async fn number(&self) -> &str {
    &self.number
  }

  pub async fn category(&self) -> CardCategory {
    self.category
  }

  pub async fn wave(&self, ctx: &Context<'_>) -> FieldResult<Wave> {
    let db = ctx.data::<Database>();
    let wave = db.get_wave(self.wave_id).await?;
    Ok(wave)
  }

  pub async fn title(&self) -> &str {
    &self.title
  }

  pub async fn stars(&self) -> Option<i32> {
    self.stars
  }

  pub async fn icons(&self) -> &Vec<BattleIcon> {
    &self.icons
  }

  pub async fn type_(&self) -> BattleType {
    self.type_
  }

  pub async fn faction(&self) -> Option<Faction> {
    self.faction
  }

  pub async fn attack_modifier(&self) -> Option<i32> {
    self.attack_modifier
  }

  pub async fn defense_modifier(&self) -> Option<i32> {
    self.defense_modifier
  }
}
