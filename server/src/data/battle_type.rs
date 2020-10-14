use async_graphql::Enum;

#[derive(Debug, Copy, Clone, Eq, PartialEq, sqlx::Type, Enum)]
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
    #[sqlx(rename = "UPGRADE_WEAPON_ARMOR")]
    UpgradeWeaponArmor,
}
