// From https://github.com/diesel-rs/diesel/blob/1.4.x/diesel_tests/tests/custom_types.rs

use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::serialize::{self, IsNull, Output, ToSql};
use std::io::Write;

#[derive(Debug, PartialEq, Eq, FromSqlRow, AsExpression, juniper::GraphQLEnum, SqlType)]
#[postgres(type_name = "BATTLE_TYPE")]
pub enum BattleType {
  Action,
  SecretAction,
  UpgradeWeapon,
  UpgradeArmor,
  UpgradeUtility,
}

impl ToSql<BattleType, Pg> for BattleType {
  fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
    match *self {
      BattleType::Action => out.write_all(b"ACTION")?,
      BattleType::SecretAction => out.write_all(b"SECRET_ACTION")?,
      BattleType::UpgradeWeapon => out.write_all(b"UPGRADE_WEAPON")?,
      BattleType::UpgradeArmor => out.write_all(b"UPGRADE_ARMOR")?,
      BattleType::UpgradeUtility => out.write_all(b"UPGRADE_UTILITY")?,
    }
    Ok(IsNull::No)
  }
}

impl FromSql<BattleType, Pg> for BattleType {
  fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
    match not_none!(bytes) {
      b"ACTION" => Ok(BattleType::Action),
      b"SECRET_ACTION" => Ok(BattleType::SecretAction),
      b"UPGRADE_WEAPON" => Ok(BattleType::UpgradeWeapon),
      b"UPGRADE_ARMOR" => Ok(BattleType::UpgradeArmor),
      b"UPGRADE_UTILITY" => Ok(BattleType::UpgradeUtility),
      // unimplemented!() is used here to give immediate feedback that
      // I forgot to deserialize the type from a Postgres type.
      not_implemented => unimplemented!(
        "Unrecognized BattleType variant: {}",
        String::from_utf8_lossy(not_implemented)
      ),
    }
  }
}
