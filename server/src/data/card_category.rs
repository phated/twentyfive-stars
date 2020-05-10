#[async_graphql::Enum]
#[derive(Debug, Copy, Clone, sqlx::Type)]
#[sqlx(rename = "CARD_CATEGORY")]
pub enum CardCategory {
  #[sqlx(rename = "CHARACTER")]
  Character,
  #[sqlx(rename = "BATTLE")]
  Battle,
  #[sqlx(rename = "STRATAGEM")]
  Stratagem,
}
