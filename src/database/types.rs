// From https://github.com/diesel-rs/diesel/blob/1.4.x/diesel_tests/tests/custom_types.rs

use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::serialize::{self, IsNull, Output, ToSql};
use std::io::Write;

#[derive(Debug, PartialEq, Eq, FromSqlRow, AsExpression, juniper::GraphQLEnum, SqlType)]
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

#[derive(Debug, PartialEq, Eq, FromSqlRow, AsExpression, juniper::GraphQLEnum, SqlType)]
#[postgres(type_name = "CARD_CATEGORY")]
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
      _ => Err("Unrecognized enum variant".into()),
    }
  }
}

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
