// From https://github.com/diesel-rs/diesel/blob/1.4.x/diesel_tests/tests/custom_types.rs

use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::serialize::{self, IsNull, Output, ToSql};
use std::io::Write;

#[async_graphql::Enum]
#[derive(Debug, PartialEq, Eq, Copy, Clone, FromSqlRow, AsExpression, QueryId, SqlType)]
#[postgres(type_name = "CARD_CATEGORY")]
#[sql_type = "CardCategory"]
pub enum CardCategory {
  Character,
  Battle,
  Stratagem,
}

impl ToSql<CardCategory, Pg> for CardCategory {
  fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
    match *self {
      CardCategory::Character => out.write_all(b"CHARACTER")?,
      CardCategory::Battle => out.write_all(b"BATTLE")?,
      CardCategory::Stratagem => out.write_all(b"STRATAGEM")?,
    }
    Ok(IsNull::No)
  }
}

impl FromSql<CardCategory, Pg> for CardCategory {
  fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
    match not_none!(bytes) {
      b"CHARACTER" => Ok(CardCategory::Character),
      b"BATTLE" => Ok(CardCategory::Battle),
      b"STRATAGEM" => Ok(CardCategory::Stratagem),
      // unimplemented!() is used here to give immediate feedback that
      // I forgot to deserialize the type from a Postgres type.
      not_implemented => unimplemented!(
        "Unrecognized CardCategory variant: {}",
        String::from_utf8_lossy(not_implemented)
      ),
    }
  }
}
