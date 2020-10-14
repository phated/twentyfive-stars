use async_graphql::Enum;

#[derive(Debug, Copy, Clone, Eq, PartialEq, sqlx::Type, Enum)]
#[sqlx(rename = "FACTION", rename_all = "uppercase")]
pub enum Faction {
    Autobot,
    Decepticon,
    Mercenary,
}
