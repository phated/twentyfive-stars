use diesel_derive_newtype::DieselNewType;
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, DieselNewType, juniper::GraphQLScalarValue)]
pub struct ID(Uuid);

impl ID {
  pub fn empty() -> Self {
    ID(Uuid::nil())
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
