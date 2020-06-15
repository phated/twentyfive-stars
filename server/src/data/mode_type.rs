use async_graphql::GQLEnum;

#[derive(Debug, Copy, Clone, Eq, PartialEq, sqlx::Type, GQLEnum)]
#[sqlx(rename = "MODE_TYPE", rename_all = "uppercase")]
#[graphql(name = "ModeType")]
pub enum ModeType {
    Alt,
    #[sqlx(rename = "ALT_1")]
    Alt1,
    #[sqlx(rename = "ALT_2")]
    Alt2,
    Bot,
    Combiner,
    #[sqlx(rename = "UPGRADE_WEAPON")]
    UpgradeWeapon,
    #[sqlx(rename = "UPGRADE_ARMOR")]
    UpgradeArmor,
    #[sqlx(rename = "UPGRADE_UTILITY")]
    UpgradeUtility,
    Body,
    Head,
    #[sqlx(rename = "COMBINER_BODY")]
    CombinerBody,
}
