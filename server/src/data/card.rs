use crate::data::{BattleCard, CardCategory, CardRarity, CharacterCard, StratagemCard, Wave, ID};
use crate::database_schema::cards_with_pageinfo;

#[async_graphql::SimpleObject]
#[derive(Debug, Clone)]
pub struct Card {
  pub id: ID,
  // pub tcg_id: String,
  // pub rarity: CardRarity,
  // pub number: String,
  pub category: CardCategory,
  // pub wave_id: ID,
  // pub sort_order: i32,
  pub has_previous: bool,
  pub has_next: bool,
}
