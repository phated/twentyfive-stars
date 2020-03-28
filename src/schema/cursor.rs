use diesel_derive_newtype::DieselNewType;
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, DieselNewType, juniper::GraphQLScalarValue)]
pub struct Cursor(Uuid);

impl Cursor {
  pub fn empty() -> Cursor {
    Cursor(Uuid::nil())
  }

  pub fn new<S: Into<Uuid>>(value: S) -> Self {
    Cursor(value.into())
  }
}
