use async_graphql::Enum;

#[derive(Debug, Copy, Clone, Eq, PartialEq, sqlx::Type, Enum)]
#[sqlx(rename = "CARD_CATEGORY", rename_all = "uppercase")]
pub enum CardCategory {
    Character,
    Battle,
    Stratagem,
}
