use async_graphql::GQLEnum;

#[derive(Debug, Copy, Clone, Eq, PartialEq, sqlx::Type, GQLEnum)]
#[sqlx(rename = "CARD_CATEGORY", rename_all = "uppercase")]
pub enum CardCategory {
    Character,
    Battle,
    Stratagem,
}
