#[async_graphql::Enum]
#[derive(Debug, Copy, Clone, sqlx::Type)]
#[sqlx(rename = "CARD_RARITY")]
#[sqlx(rename_all = "uppercase")]
pub enum CardRarity {
    Common,
    Uncommon,
    Rare,
    #[sqlx(rename = "SUPER_RARE")]
    SuperRare,
    Theme,
    Promo,
}
