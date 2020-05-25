#[async_graphql::Enum]
#[derive(Debug, Copy, Clone, sqlx::Type)]
#[sqlx(rename = "FACTION")]
#[sqlx(rename_all = "uppercase")]
pub enum Faction {
    Autobot,
    Decepticon,
    Mercenary,
}
