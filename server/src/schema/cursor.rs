use async_graphql::{Result, Scalar, ScalarType, Value};
use diesel_derive_newtype::DieselNewType;
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, DieselNewType)]
pub struct Cursor(Uuid);

impl Cursor {
    pub fn empty() -> Cursor {
        Cursor(Uuid::nil())
    }

    pub fn new<S: Into<Uuid>>(value: S) -> Self {
        Cursor(value.into())
    }
}

impl Into<String> for Cursor {
    fn into(self) -> String {
        self.0.to_string()
    }
}

#[Scalar]
impl ScalarType for Cursor {
    fn type_name() -> &'static str {
        "Cursor"
    }

    fn parse(value: &Value) -> Option<Self> {
        match value {
            Value::String(s) => Some(Cursor(Uuid::parse_str(s).ok()?)),
            _ => None,
        }
    }

    fn to_json(&self) -> Result<serde_json::Value> {
        Ok(self.0.to_string().into())
    }
}
