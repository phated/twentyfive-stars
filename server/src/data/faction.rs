use async_graphql::GQLEnum;

#[derive(Debug, Copy, Clone, Eq, PartialEq, sqlx::Type, GQLEnum)]
#[sqlx(rename = "FACTION", rename_all = "uppercase")]
pub enum Faction {
    Autobot,
    Decepticon,
    Mercenary,
}
