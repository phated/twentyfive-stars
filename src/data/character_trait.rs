// From https://github.com/diesel-rs/diesel/blob/1.4.x/diesel_tests/tests/custom_types.rs

use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::serialize::{self, IsNull, Output, ToSql};
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, FromSqlRow, AsExpression, juniper::GraphQLEnum, SqlType)]
#[postgres(type_name = "CHARACTER_TRAIT")]
pub enum CharacterTrait {
  Melee,
  Ranged,
  Specialist,
}

impl ToSql<CharacterTrait, Pg> for CharacterTrait {
  fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
    match *self {
      CharacterTrait::Melee => out.write_all(b"MELEE")?,
      CharacterTrait::Ranged => out.write_all(b"RANGED")?,
      CharacterTrait::Specialist => out.write_all(b"SPECIALIST")?,
    }
    Ok(IsNull::No)
  }
}

impl FromSql<CharacterTrait, Pg> for CharacterTrait {
  fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
    match not_none!(bytes) {
      b"MELEE" => Ok(CharacterTrait::Melee),
      b"RANGED" => Ok(CharacterTrait::Ranged),
      b"SPECIALIST" => Ok(CharacterTrait::Specialist),
      _ => Err("Unrecognized enum variant".into()),
    }
  }
}
