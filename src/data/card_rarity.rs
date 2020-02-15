// From https://github.com/diesel-rs/diesel/blob/1.4.x/diesel_tests/tests/custom_types.rs

use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::serialize::{self, IsNull, Output, ToSql};
use std::io::Write;

#[derive(Debug, PartialEq, Eq, Clone, FromSqlRow, AsExpression, juniper::GraphQLEnum, SqlType)]
#[postgres(type_name = "CARD_RARITY")]
pub enum CardRarity {
  Common,
  Uncommon,
  Rare,
  SuperRare,
  Theme,
  Promo,
}

impl ToSql<CardRarity, Pg> for CardRarity {
  fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
    match *self {
      CardRarity::Common => out.write_all(b"COMMON")?,
      CardRarity::Uncommon => out.write_all(b"UNCOMMON")?,
      CardRarity::Rare => out.write_all(b"RARE")?,
      CardRarity::SuperRare => out.write_all(b"SUPER_RARE")?,
      CardRarity::Theme => out.write_all(b"THEME")?,
      CardRarity::Promo => out.write_all(b"PROMO")?,
    }
    Ok(IsNull::No)
  }
}

impl FromSql<CardRarity, Pg> for CardRarity {
  fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
    match not_none!(bytes) {
      b"COMMON" => Ok(CardRarity::Common),
      b"UNCOMMON" => Ok(CardRarity::Uncommon),
      b"RARE" => Ok(CardRarity::Rare),
      b"SUPER_RARE" => Ok(CardRarity::SuperRare),
      b"THEME" => Ok(CardRarity::Theme),
      b"PROMO" => Ok(CardRarity::Promo),
      _ => Err("Unrecognized enum variant".into()),
    }
  }
}
