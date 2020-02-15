use crate::data::{Card, CardCategory, CardRarity, CharacterMode, Wave};
use crate::graphql_schema::Context;
use uuid::Uuid;

pub struct CharacterCard {
  pub modes: Vec<CharacterMode>,
}

juniper::graphql_object!(CharacterCard: Context | &self | {
  interfaces: [&Card]

  field modes() -> &Vec<CharacterMode> {
    &self.modes
  }

  // Implemented by the interface
  field id() -> Uuid {
    unimplemented!("`id` field should be implemented by interface")
  }

  field tcg_id() -> &str {
    unimplemented!("`tcg_id` field should be implemented by interface")
  }

  field rarity() -> &CardRarity {
    unimplemented!("`rarity` field should be implemented by interface")
  }

  field number() -> &str {
    unimplemented!("`number` field should be implemented by interface")
  }

  field category() -> &CardCategory {
    unimplemented!("`category` field should be implemented by interface")
  }

  field wave() -> Wave {
    unimplemented!("`wave` field should be implemented by interface")
  }
});
