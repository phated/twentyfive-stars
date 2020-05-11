use async_graphql::{Cursor, InputValueError, InputValueResult, Result, Scalar, ScalarType, Value};
use diesel_derive_newtype::DieselNewType;
use serde_json;
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, DieselNewType)]
pub struct ID(Uuid);

#[Scalar]
impl ScalarType for ID {
    fn type_name() -> &'static str {
        "ID"
    }

    fn parse(value: &Value) -> InputValueResult<Self> {
        match value {
            Value::String(s) => Ok(Uuid::parse_str(&s).map(ID)?),
            _ => Err(InputValueError::ExpectedType),
        }
    }

    fn to_json(&self) -> Result<serde_json::Value> {
        Ok(self.0.to_string().into())
    }
}

impl From<Uuid> for ID {
    fn from(uuid: Uuid) -> ID {
        ID(uuid)
    }
}

impl Into<Uuid> for ID {
    fn into(self) -> Uuid {
        self.0
    }
}

impl Into<Cursor> for ID {
    // TODO: Into<Cursor> for Uuid
    fn into(self) -> Cursor {
        self.0.to_string().into()
    }
}
