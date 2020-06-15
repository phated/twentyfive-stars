use async_graphql::GQLEnum;

#[derive(Debug, Copy, Clone, Eq, PartialEq, sqlx::Type, GQLEnum)]
#[sqlx(rename = "text", rename_all = "uppercase")]
#[graphql(name = "BattleIcon")]
pub enum BattleIcon {
    Orange,
    Blue,
    White,
    Green,
    Black,
    #[sqlx(rename = "ORANGE_TANK")]
    OrangeTank,
    #[sqlx(rename = "BLUE_PLANE")]
    BluePlane,
    #[sqlx(rename = "BLACK_TITAN_MASTER")]
    BlackTitanMaster,
    #[sqlx(rename = "ORANGE_SPECIALIST")]
    OrangeSpecialist,
    #[sqlx(rename = "BLACK_RANGED")]
    BlackRanged,
    #[sqlx(rename = "BLUE_MELEE")]
    BlueMelee,
    #[sqlx(rename = "BLACK_CAR")]
    BlackCar,
    #[sqlx(rename = "GREEN_RANGED")]
    GreenRanged,
    #[sqlx(rename = "BLACK_SPECIALIST")]
    BlackSpecialist,
    #[sqlx(rename = "BLACK_PLANE")]
    BlackPlane,
}
