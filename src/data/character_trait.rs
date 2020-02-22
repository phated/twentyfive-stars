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
  Motorcycle,
  Spaceship,
  Truck,
  Leader,
  Car,
  Insecticon,
  Tank,
  Dinobot,
  Plane,
}

impl ToSql<CharacterTrait, Pg> for CharacterTrait {
  fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
    match *self {
      CharacterTrait::Melee => out.write_all(b"MELEE")?,
      CharacterTrait::Ranged => out.write_all(b"RANGED")?,
      CharacterTrait::Specialist => out.write_all(b"SPECIALIST")?,
      CharacterTrait::Motorcycle => out.write_all(b"MOTORCYCLE")?,
      CharacterTrait::Spaceship => out.write_all(b"SPACESHIP")?,
      CharacterTrait::Truck => out.write_all(b"TRUCK")?,
      CharacterTrait::Leader => out.write_all(b"LEADER")?,
      CharacterTrait::Car => out.write_all(b"CAR")?,
      CharacterTrait::Insecticon => out.write_all(b"INSECTICON")?,
      CharacterTrait::Tank => out.write_all(b"TANK")?,
      CharacterTrait::Dinobot => out.write_all(b"DINOBOT")?,
      CharacterTrait::Plane => out.write_all(b"PLANE")?,
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
      b"MOTORCYCLE" => Ok(CharacterTrait::Motorcycle),
      b"SPACESHIP" => Ok(CharacterTrait::Spaceship),
      b"TRUCK" => Ok(CharacterTrait::Truck),
      b"LEADER" => Ok(CharacterTrait::Leader),
      b"CAR" => Ok(CharacterTrait::Car),
      b"INSECTICON" => Ok(CharacterTrait::Insecticon),
      b"TANK" => Ok(CharacterTrait::Tank),
      b"DINOBOT" => Ok(CharacterTrait::Dinobot),
      b"PLANE" => Ok(CharacterTrait::Plane),
      // unimplemented!() is used here to give immediate feedback that
      // I forgot to deserialize the type from a Postgres type.
      not_implemented => unimplemented!(
        "Unrecognized CharacterTrait variant: {}",
        String::from_utf8_lossy(not_implemented)
      ),
    }
  }
}
