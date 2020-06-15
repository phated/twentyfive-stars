use async_graphql::GQLEnum;

#[derive(Debug, Copy, Clone, Eq, PartialEq, sqlx::Type, GQLEnum)]
#[sqlx(rename = "BATTLE_TYPE")]
#[graphql(name = "BattleType")]
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
