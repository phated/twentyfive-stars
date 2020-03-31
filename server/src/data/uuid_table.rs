// From https://github.com/diesel-rs/diesel/blob/1.4.x/diesel_tests/tests/custom_types.rs

use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::serialize::{self, IsNull, Output, ToSql};
use std::io::Write;

#[derive(Debug, PartialEq, Eq, FromSqlRow, AsExpression, SqlType)]
#[postgres(type_name = "UUID_TABLE")]
pub enum UuidTable {
  Waves,
  Cards,
  // BattleCards,
  // CharacterModes,
  // StratagemCards,
}

impl ToSql<UuidTable, Pg> for UuidTable {
  fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
    match *self {
      UuidTable::Waves => out.write_all(b"WAVES")?,
      UuidTable::Cards => out.write_all(b"CARDS")?,
      // UuidTable::BattleCards => out.write_all(b"BATTLE_CARDS")?,
      // UuidTable::CharacterModes => out.write_all(b"CHARACTER_MODES")?,
      // UuidTable::StratagemCards => out.write_all(b"STRATAGEM_CARDS")?,
    }
    Ok(IsNull::No)
  }
}

impl FromSql<UuidTable, Pg> for UuidTable {
  fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
    match not_none!(bytes) {
      b"WAVES" => Ok(UuidTable::Waves),
      b"CARDS" => Ok(UuidTable::Cards),
      // b"BATTLE_CARDS" => Ok(UuidTable::BattleCards),
      // b"CHARACTER_MODES" => Ok(UuidTable::CharacterModes),
      // b"STRATAGEM_CARDS" => Ok(UuidTable::StratagemCards),
      // unimplemented!() is used here to give immediate feedback that
      // I forgot to deserialize the type from a Postgres type.
      not_implemented => unimplemented!(
        "Unrecognized UuidTable variant: {}",
        String::from_utf8_lossy(not_implemented)
      ),
    }
  }
}
