use crate::data::{BattleIcon, BattleType, Card, CardCategory, CardRarity, Faction, Wave, ID};
use crate::database::{get_wave, ConnPool};
use crate::database_schema::battle_cards;
use crate::schema::Cursor;
use async_graphql::{Context, FieldResult};
use diesel::prelude::*;
use tokio_diesel::*;

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

  pub async fn load_from_card(card: Card, pool: &ConnPool) -> AsyncResult<BattleCard> {
    battle_cards::table
      .filter(battle_cards::card_id.eq(card.id))
      .first_async::<ExtraProps>(&pool)
      .await
      // TODO: performance of cloning this?
      .map(|extra| BattleCard::new(card.clone(), extra))
  }
}

#[async_graphql::Object]
impl BattleCard {
  pub fn cursor(&self) -> Cursor {
    Cursor::new(self.0.id)
  }

  #[field]
  pub async fn id(&self) -> ID {
    self.0.id
  }

  #[field]
  pub async fn tcg_id(&self) -> &str {
    &self.0.tcg_id
  }

  #[field]
  pub async fn rarity(&self) -> CardRarity {
    self.0.rarity
  }

  #[field]
  pub async fn number(&self) -> &str {
    &self.0.number
  }

  #[field]
  pub async fn category(&self) -> CardCategory {
    self.0.category
  }

  #[field]
  pub async fn wave(&self, ctx: &Context<'_>) -> FieldResult<Wave> {
    let pool = ctx.data::<ConnPool>();
    let wave = get_wave(pool, self.0.wave_id).await?;
    Ok(wave)
  }

  #[field]
  pub async fn title(&self) -> &str {
    &self.1.title
  }

  #[field]
  pub async fn stars(&self) -> Option<i32> {
    self.1.stars
  }

  #[field]
  pub async fn icons(&self) -> &Vec<BattleIcon> {
    &self.1.icons
  }

  #[field]
  pub async fn type_(&self) -> BattleType {
    self.1.type_
  }

  #[field]
  pub async fn faction(&self) -> Option<Faction> {
    self.1.faction
  }

  #[field]
  pub async fn attack_modifier(&self) -> Option<i32> {
    self.1.attack_modifier
  }

  #[field]
  pub async fn defense_modifier(&self) -> Option<i32> {
    self.1.defense_modifier
  }
}
