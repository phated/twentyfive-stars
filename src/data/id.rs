use base64::{decode, encode};
use juniper::{graphql_value, FieldError};
use juniper::{ParseScalarResult, ParseScalarValue, Value};
use std::fmt;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug)]
pub enum ID {
  CardID(Uuid),
  WaveID(Uuid),
  BattleCardID(Uuid),
  CharacterModeID(Uuid),
  StratagemCardID(Uuid),
}

impl ID {
  pub fn raw(id: ID) -> Uuid {
    match id {
      ID::CardID(uuid)
      | ID::WaveID(uuid)
      | ID::BattleCardID(uuid)
      | ID::CharacterModeID(uuid)
      | ID::StratagemCardID(uuid) => uuid,
    }
  }
}

impl fmt::Display for ID {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let formatted = match self {
      ID::CardID(id) => format!("{}|{}", 0, id),
      ID::WaveID(id) => format!("{}|{}", 1, id),
      ID::BattleCardID(id) => format!("{}|{}", 2, id),
      ID::CharacterModeID(id) => format!("{}|{}", 3, id),
      ID::StratagemCardID(id) => format!("{}|{}", 4, id),
    };
    write!(f, "{}", encode(&*formatted))
  }
}

impl FromStr for ID {
  type Err = FieldError;

  fn from_str(id: &str) -> Result<Self, Self::Err> {
    let base64 = decode(&id).map_err(|err| {
      FieldError::new(
        "ID parsing error",
        graphql_value!({ "code": 300, "details": { err.to_string() }}),
      )
    })?;

    let id = String::from_utf8(base64).map_err(|err| {
      FieldError::new(
        "ID parsing error",
        graphql_value!({ "code": 300, "details": { err.to_string() }}),
      )
    })?;

    let v: Vec<&str> = id.split('|').collect();
    match v.as_slice() {
      ["0", id] => {
        // TODO: will these bubble weird errors?
        let id = Uuid::parse_str(id)?;
        Ok(ID::CardID(id))
      }
      ["1", id] => {
        let id = Uuid::parse_str(id)?;
        Ok(ID::WaveID(id))
      }
      ["2", id] => {
        let id = Uuid::parse_str(id)?;
        Ok(ID::BattleCardID(id))
      }
      ["3", id] => {
        let id = Uuid::parse_str(id)?;
        Ok(ID::CharacterModeID(id))
      }
      ["4", id] => {
        let id = Uuid::parse_str(id)?;
        Ok(ID::StratagemCardID(id))
      }
      _ => Err(FieldError::new(
        "ID parsing error",
        graphql_value!({ "code": 300, "details": { "cannot resolve id" }}),
      )),
    }
  }
}

// TODO: I don't understand this at all, just copied from docs
juniper::graphql_scalar!(ID where Scalar = <S> {
  description: "An opaque identifier, represented as a string"

  resolve(&self) -> Value {
    Value::scalar(self.to_string())
  }

  from_input_value(v: &InputValue) -> Option<ID> {
    // TODO: I have no idea why the other way fails
    #[allow(deprecated)]
    v.as_string_value().and_then(|s| s.parse().ok())
  }

  from_str<'a>(value: ScalarToken<'a>) -> ParseScalarResult<'a, S> {
    <String as ParseScalarValue<S>>::from_str(value)
  }
});
