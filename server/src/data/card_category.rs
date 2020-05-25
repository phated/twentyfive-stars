#[async_graphql::Enum]
#[derive(Debug, Copy, Clone, sqlx::Type)]
#[sqlx(rename = "CARD_CATEGORY")]
#[sqlx(rename_all = "uppercase")]
pub enum CardCategory {
    Character,
    Battle,
    Stratagem,
}
