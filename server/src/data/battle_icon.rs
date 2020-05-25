#[async_graphql::Enum]
#[derive(Debug, Copy, Clone, sqlx::Type)]
#[sqlx(rename = "text")]
#[sqlx(rename_all = "uppercase")]
pub enum BattleIcon {
    Orange,
    Blue,
    White,
    Green,
    Black,
}
