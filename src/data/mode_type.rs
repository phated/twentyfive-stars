// From https://github.com/diesel-rs/diesel/blob/1.4.x/diesel_tests/tests/custom_types.rs

use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::serialize::{self, IsNull, Output, ToSql};
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, FromSqlRow, AsExpression, juniper::GraphQLEnum, SqlType)]
#[postgres(type_name = "MODE_TYPE")]
pub enum ModeType {
  Alt,
  Alt1,
  Alt2,
  Bot,
  // Combiner,
  // Body,
  // Head,
  // UpgradeWeapon,
  // UpgradeArmor,
  // UpgradeUtility,
}

impl ToSql<ModeType, Pg> for ModeType {
  fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
    match *self {
      ModeType::Alt => out.write_all(b"ALT")?,
      ModeType::Alt1 => out.write_all(b"ALT_1")?,
      ModeType::Alt2 => out.write_all(b"ALT_2")?,
      ModeType::Bot => out.write_all(b"BOT")?,
      // ModeType::Combiner => out.write_all(b"COMBINER")?,
      // ModeType::Body => out.write_all(b"BODY")?,
      // ModeType::Head => out.write_all(b"HEAD")?,
      // ModeType::UpgradeWeapon => out.write_all(b"UPGRADE_WEAPON")?,
      // ModeType::UpgradeArmor => out.write_all(b"UPGRADE_ARMOR")?,
      // ModeType::UpgradeUtility => out.write_all(b"UPGRADE_UTILITY")?,
    }
    Ok(IsNull::No)
  }
}

impl FromSql<ModeType, Pg> for ModeType {
  fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
    match not_none!(bytes) {
      b"ALT" => Ok(ModeType::Alt),
      b"ALT_1" => Ok(ModeType::Alt1),
      b"ALT_2" => Ok(ModeType::Alt2),
      b"BOT" => Ok(ModeType::Bot),
      // b"COMBINER" => Ok(ModeType::Combiner),
      // b"BODY" => Ok(ModeType::Body),
      // b"HEAD" => Ok(ModeType::Head),
      // b"UPGRADE_WEAPON" => Ok(ModeType::UpgradeWeapon),
      // b"UPGRADE_ARMOR" => Ok(ModeType::UpgradeArmor),
      // b"UPGRADE_UTILITY" => Ok(ModeType::UpgradeUtility),
      _ => Err("Unrecognized enum variant".into()),
    }
  }
}
