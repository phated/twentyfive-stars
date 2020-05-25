#[async_graphql::Enum]
#[derive(Debug, Copy, Clone, sqlx::Type)]
#[sqlx(rename = "BATTLE_TYPE")]
// TODO: #[sqlx(rename_all = ["snake_case", "uppercase"])]
pub enum BattleType {
    #[sqlx(rename = "ACTION")]
    Action,
    #[sqlx(rename = "SECRET_ACTION")]
    SecretAction,
    #[sqlx(rename = "UPGRADE_WEAPON")]
    UpgradeWeapon,
    #[sqlx(rename = "UPGRADE_ARMOR")]
    UpgradeArmor,
    #[sqlx(rename = "UPGRADE_UTILITY")]
    UpgradeUtility,
}
