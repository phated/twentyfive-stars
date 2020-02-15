// From https://github.com/diesel-rs/diesel/blob/1.4.x/diesel_tests/tests/custom_types.rs

use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::serialize::{self, IsNull, Output, ToSql};
use std::io::Write;

#[derive(Debug, PartialEq, Eq, FromSqlRow, AsExpression, juniper::GraphQLEnum, SqlType)]
#[postgres(type_name = "BATTLE_ICON")]
pub enum BattleIcon {
  Orange,
  Blue,
  White,
  Green,
  Black,
}

impl ToSql<BattleIcon, Pg> for BattleIcon {
  fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
    match *self {
      BattleIcon::Orange => out.write_all(b"ORANGE")?,
      BattleIcon::Blue => out.write_all(b"BLUE")?,
      BattleIcon::White => out.write_all(b"WHITE")?,
      BattleIcon::Green => out.write_all(b"GREEN")?,
      BattleIcon::Black => out.write_all(b"BLACK")?,
    }
    Ok(IsNull::No)
  }
}

impl FromSql<BattleIcon, Pg> for BattleIcon {
  fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
    match not_none!(bytes) {
      b"ORANGE" => Ok(BattleIcon::Orange),
      b"BLUE" => Ok(BattleIcon::Blue),
      b"WHITE" => Ok(BattleIcon::White),
      b"GREEN" => Ok(BattleIcon::Green),
      b"BLACK" => Ok(BattleIcon::Black),
      _ => Err("Unrecognized enum variant".into()),
    }
  }
}
