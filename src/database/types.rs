// From https://github.com/diesel-rs/diesel/blob/1.4.x/diesel_tests/tests/custom_types.rs

use crate::database::schema::*;
use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
// use diesel::prelude::*;
use diesel::serialize::{self, IsNull, Output, ToSql};
use std::io::Write;
use uuid::Uuid;

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

#[derive(Debug, PartialEq, Eq, FromSqlRow, AsExpression, QueryId, juniper::GraphQLEnum, SqlType)]
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

#[derive(Debug, Clone, PartialEq, Eq, FromSqlRow, AsExpression, juniper::GraphQLEnum, SqlType)]
#[postgres(type_name = "MODE_TYPE")]
pub enum ModeType {
  Alt,
  Alt1,
  Alt2,
  Bot,
  Combiner,
  Body,
  Head,
  UpgradeWeapon,
  UpgradeArmor,
  UpgradeUtility,
}

impl ToSql<ModeType, Pg> for ModeType {
  fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
    match *self {
      ModeType::Alt => out.write_all(b"ALT")?,
      ModeType::Alt1 => out.write_all(b"ALT_1")?,
      ModeType::Alt2 => out.write_all(b"ALT_2")?,
      ModeType::Bot => out.write_all(b"BOT")?,
      ModeType::Combiner => out.write_all(b"COMBINER")?,
      ModeType::Body => out.write_all(b"BODY")?,
      ModeType::Head => out.write_all(b"HEAD")?,
      ModeType::UpgradeWeapon => out.write_all(b"UPGRADE_WEAPON")?,
      ModeType::UpgradeArmor => out.write_all(b"UPGRADE_ARMOR")?,
      ModeType::UpgradeUtility => out.write_all(b"UPGRADE_UTILITY")?,
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
      b"COMBINER" => Ok(ModeType::Combiner),
      b"BODY" => Ok(ModeType::Body),
      b"HEAD" => Ok(ModeType::Head),
      b"UPGRADE_WEAPON" => Ok(ModeType::UpgradeWeapon),
      b"UPGRADE_ARMOR" => Ok(ModeType::UpgradeArmor),
      b"UPGRADE_UTILITY" => Ok(ModeType::UpgradeUtility),
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

#[derive(Identifiable, Queryable, Associations, PartialEq, Eq, Debug)]
pub struct Card {
  pub id: Uuid,
  pub tcg_id: String,
  pub rarity: CardRarity,
  pub number: String,
  pub category: CardCategory,
  pub wave_id: Uuid,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Eq, Debug)]
#[belongs_to(Card)]
pub struct CharacterMode {
  pub id: Uuid,
  pub card_id: Uuid,
  pub title: String,
  pub subtitle: Option<String>,
  pub traits: Vec<CharacterTrait>,
  pub type_: ModeType,
  pub stars: i32,
  pub health: Option<i32>,
  pub attack: Option<i32>,
  pub defense: Option<i32>,
  pub attack_modifier: Option<i32>,
  pub defense_modifier: Option<i32>,
}
