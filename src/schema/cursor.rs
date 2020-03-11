use base64::{decode, encode};
use juniper::{graphql_value, FieldError};
use juniper::{ParseScalarResult, ParseScalarValue, Value};
use std::fmt;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Copy, Clone, Debug)]
pub struct Cursor(pub Uuid);

impl Cursor {
  pub fn empty() -> Self {
    Cursor(Uuid::nil())
  }
}

impl fmt::Display for Cursor {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let formatted = match self {
      Cursor(id) => format!("{}|{}", 0, id),
    };
    write!(f, "{}", encode(&*formatted))
  }
}

impl FromStr for Cursor {
  type Err = FieldError;

  fn from_str(cursor: &str) -> Result<Self, Self::Err> {
    let base64 = decode(&cursor).map_err(|err| {
      FieldError::new(
        "Cursor parsing error",
        graphql_value!({ "code": 300, "details": { err.to_string() }}),
      )
    })?;

    let cursor = String::from_utf8(base64).map_err(|err| {
      FieldError::new(
        "Cursor parsing error",
        graphql_value!({ "code": 300, "details": { err.to_string() }}),
      )
    })?;

    let v: Vec<&str> = cursor.split('|').collect();
    match v.as_slice() {
      // TODO: will these bubble weird errors?
      [_, id] => {
        let id = Uuid::parse_str(id)?;
        Ok(Cursor(id))
      }
      // ["1", offset] => Ok(Cursor::Descending(i32::from_str_radix(offset, 10)?)),
      _ => Err(FieldError::new(
        "Cursor parsing error",
        graphql_value!({ "code": 300, "details": { "cannot resolve cursor" }}),
      )),
    }
  }
}

// TODO: I don't understand this at all, just copied from docs
juniper::graphql_scalar!(Cursor where Scalar = <S> {
  description: "An opaque identifier for pagination, represented as a string"

  resolve(&self) -> Value {
    Value::scalar(self.to_string())
  }

  from_input_value(v: &InputValue) -> Option<Cursor> {
    // TODO: I have no idea why the other way fails
    #[allow(deprecated)]
    v.as_string_value().and_then(|s| s.parse().ok())
  }

  from_str<'a>(value: ScalarToken<'a>) -> ParseScalarResult<'a, S> {
    <String as ParseScalarValue<S>>::from_str(value)
  }
});
