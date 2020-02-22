// From https://github.com/diesel-rs/diesel/blob/1.4.x/diesel_tests/tests/custom_types.rs

use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::serialize::{self, IsNull, Output, ToSql};
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, FromSqlRow, AsExpression, juniper::GraphQLEnum, SqlType)]
#[postgres(type_name = "FACTION")]
pub enum Faction {
  Autobot,
  Decepticon,
  Mercenary,
}

impl ToSql<Faction, Pg> for Faction {
  fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
    match *self {
      Faction::Autobot => out.write_all(b"AUTOBOT")?,
      Faction::Decepticon => out.write_all(b"DECEPTICON")?,
      Faction::Mercenary => out.write_all(b"MERCENARY")?,
    }
    Ok(IsNull::No)
  }
}

impl FromSql<Faction, Pg> for Faction {
  fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
    match not_none!(bytes) {
      b"AUTOBOT" => Ok(Faction::Autobot),
      b"DECEPTICON" => Ok(Faction::Decepticon),
      b"MERCENARY" => Ok(Faction::Mercenary),
      // unimplemented!() is used here to give immediate feedback that
      // I forgot to deserialize the type from a Postgres type.
      not_implemented => unimplemented!(
        "Unrecognized Faction variant: {}",
        String::from_utf8_lossy(not_implemented)
      ),
    }
  }
}
