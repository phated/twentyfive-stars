// From https://github.com/diesel-rs/diesel/blob/1.4.x/diesel_tests/tests/custom_types.rs

use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::serialize::{self, IsNull, Output, ToSql};
use std::io::Write;

#[derive(
  Debug, PartialEq, Eq, Clone, FromSqlRow, AsExpression, QueryId, juniper::GraphQLEnum, SqlType,
)]
#[postgres(type_name = "CARD_CATEGORY")]
#[sql_type = "CardCategory"]
pub enum CardCategory {
  Character,
  Battle,
}

impl ToSql<CardCategory, Pg> for CardCategory {
  fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
    match *self {
      CardCategory::Character => out.write_all(b"CHARACTER")?,
      CardCategory::Battle => out.write_all(b"BATTLE")?,
    }
    Ok(IsNull::No)
  }
}

impl FromSql<CardCategory, Pg> for CardCategory {
  fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
    match not_none!(bytes) {
      b"CHARACTER" => Ok(CardCategory::Character),
      b"BATTLE" => Ok(CardCategory::Battle),
      _ => Err("Unrecognized enum variant".into()),
    }
  }
}
