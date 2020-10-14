use async_graphql::Enum;

#[derive(Debug, Copy, Clone, Eq, PartialEq, sqlx::Type, Enum)]
#[sqlx(rename = "CARD_RARITY", rename_all = "uppercase")]
pub enum CardRarity {
    Common,
    Uncommon,
    Rare,
    #[sqlx(rename = "SUPER_RARE")]
    SuperRare,
    Theme,
    Promo,
}
