use crate::data::{Card, CardCategory, CardRarity, Faction, Wave, ID};
use crate::database::Database;
use crate::database_schema::stratagem_cards;
use async_graphql::{Context, Cursor, FieldResult};
use diesel::prelude::*;

#[derive(Identifiable, Queryable, PartialEq, Eq, Clone, Debug)]
#[table_name = "stratagem_cards"]
pub struct StratagemCard {
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
  // Stratagem Card
  pub title: String,
  pub requirement: String,
  pub faction: Option<Faction>,
  pub stars: i32,
}

impl StratagemCard {
  // pub fn new(card: Card, extra: ExtraProps) -> Self {
  //   StratagemCard(card, extra)
  // }

  // pub fn load_from_card(card: Card, pool: &ConnPool) -> QueryResult<StratagemCard> {
  //   let conn = pool.get().unwrap();

  //   stratagem_cards::table
  //     .filter(stratagem_cards::card_id.eq(card.id))
  //     .first::<ExtraProps>(&conn)
  //     // TODO: performance of cloning this?
  //     .map(|extra| StratagemCard::new(card.clone(), extra))
  // }
}

impl Into<Cursor> for StratagemCard {
  fn into(self) -> Cursor {
    self.id.into()
  }
}

#[async_graphql::Object]
impl StratagemCard {
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

  pub async fn requirement(&self) -> &str {
    &self.requirement
  }

  pub async fn stars(&self) -> i32 {
    self.stars
  }

  pub async fn faction(&self) -> &Option<Faction> {
    &self.faction
  }
}
