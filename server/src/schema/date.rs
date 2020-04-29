use async_graphql::{Result, Scalar, ScalarType, Value};
use chrono::NaiveDate;
use diesel_derive_newtype::DieselNewType;
use serde_json;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, DieselNewType)]
pub struct Date(NaiveDate);

#[Scalar]
impl ScalarType for Date {
    fn type_name() -> &'static str {
        "Date"
    }

    fn parse(value: &Value) -> Option<Self> {
        match value {
            Value::String(s) => Some(Date(NaiveDate::parse_from_str(s, "%Y-%m-%d").ok()?)),
            _ => None,
        }
    }

    fn to_json(&self) -> Result<serde_json::Value> {
        Ok(self.0.format("%Y-%m-%d").to_string().into())
    }
}
