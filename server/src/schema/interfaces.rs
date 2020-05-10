use crate::data::{BattleCard, CardCategory, CardRarity, CharacterCard, StratagemCard, Wave, ID};

#[async_graphql::Interface(field(name = "id", type = "ID"))]
pub struct Node(
  BattleCard,
  CharacterCard,
  StratagemCard,
  // TODO: get methods supported on interfaces
  // CharacterMode,
  Wave,
);

#[async_graphql::Interface(
  field(name = "id", type = "ID"),
  field(name = "tcg_id", type = "&str"),
  field(name = "rarity", type = "CardRarity"),
  field(name = "number", type = "&str"),
  field(name = "category", type = "CardCategory"),
  field(name = "wave", type = "Wave")
)]
#[derive(Clone, Debug)]
pub struct Card(BattleCard, CharacterCard, StratagemCard);
