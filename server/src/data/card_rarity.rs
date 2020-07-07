use async_graphql::GQLEnum;

#[derive(Debug, Copy, Clone, Eq, PartialEq, sqlx::Type, GQLEnum)]
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
