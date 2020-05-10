use crate::data::{Card, CardCategory, CardRarity, Faction, Wave, ID};
use crate::database::Database;
use crate::database_schema::stratagem_cards;
use async_graphql::{Context, Cursor, FieldResult};

#[derive(Identifiable, Queryable, PartialEq, Eq, Clone, Debug)]
#[table_name = "stratagem_cards"]
pub struct StratagemCardProps {
  id: ID,
  card_id: ID,
  title: String,
  requirement: String,
  faction: Option<Faction>,
  stars: i32,
  sort_order: i32,
}

#[derive(Clone, Debug)]
pub struct StratagemCard(Card, StratagemCardProps);

impl StratagemCard {
  pub fn new(card: Card, extra: StratagemCardProps) -> Self {
    StratagemCard(card, extra)
  }
}

impl Into<Cursor> for StratagemCard {
  fn into(self) -> Cursor {
    self.0.id.into()
  }
}

#[async_graphql::Object]
impl StratagemCard {
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

  pub async fn requirement(&self) -> &str {
    &self.1.requirement
  }

  pub async fn stars(&self) -> i32 {
    self.1.stars
  }

  pub async fn faction(&self) -> &Option<Faction> {
    &self.1.faction
  }
}
