use crate::database::types;
use crate::database::{CharacterTrait, ModeType};
use crate::graphql_schema::Context;
use crate::models::{AltMode, BotMode};
use uuid::Uuid;

pub enum CharacterMode {
  AltMode(AltMode),
  BotMode(BotMode),
  // CombinerMode(CombinerMode),
  // BodyMode(BodyMode),
  // HeadMode(HeadMode),
  // UpgradeMode(UpgradeMode),
}

impl From<&types::CharacterMode> for CharacterMode {
  fn from(card: &types::CharacterMode) -> Self {
    match card.type_ {
      ModeType::Alt | ModeType::Alt1 | ModeType::Alt2 => CharacterMode::AltMode(AltMode {
        id: card.id,
        title: card.title.clone(),
        subtitle: card.subtitle.clone().expect("AltMode must have a subtitle"),
        stars: card.stars,
        type_: card.type_.clone(),
        traits: card.traits.to_vec(),
        health: card.health.expect("AltMode must have health"),
        attack: card.attack.expect("AltMode must have attack"),
        defense: card.defense.expect("AltMode must have defense"),
      }),
      ModeType::Bot => CharacterMode::BotMode(BotMode {
        id: card.id,
        title: card.title.clone(),
        subtitle: card.subtitle.clone().expect("BotMode must have a subtitle"),
        stars: card.stars,
        type_: card.type_.clone(),
        traits: card.traits.to_vec(),
        health: card.health.expect("BotMode must have health"),
        attack: card.attack.expect("BotMode must have attack"),
        defense: card.defense.expect("BotMode must have defense"),
      }),
      _ => CharacterMode::AltMode(AltMode {
        id: card.id,
        title: card.title.clone(),
        subtitle: card.subtitle.clone().expect("AltMode must have a subtitle"),
        stars: card.stars,
        type_: card.type_.clone(),
        traits: card.traits.to_vec(),
        health: card.health.expect("AltMode must have health"),
        attack: card.attack.expect("AltMode must have attack"),
        defense: card.defense.expect("AltMode must have defense"),
      }),
    }
  }
}

juniper::graphql_interface!(CharacterMode: Context |&self| {
  field id() -> Uuid {
    match *self {
      CharacterMode::AltMode(AltMode { id, .. }) | CharacterMode::BotMode(BotMode { id, .. }) => id,
    }
  }

  field title() -> &str {
    match *self {
      CharacterMode::AltMode(AltMode { ref title, .. }) | CharacterMode::BotMode(BotMode { ref title, .. }) => title,
    }
  }

  field stars() -> i32 {
    match *self {
      CharacterMode::AltMode(AltMode { stars, .. }) | CharacterMode::BotMode(BotMode { stars, .. }) => stars,
    }
  }
  field type_() -> &ModeType {
    match *self {
      CharacterMode::AltMode(AltMode { ref type_, .. }) | CharacterMode::BotMode(BotMode { ref type_, .. }) => type_,
    }
  }
  field traits() -> &Vec<CharacterTrait> {
    match *self {
      CharacterMode::AltMode(AltMode { ref traits, .. }) | CharacterMode::BotMode(BotMode { ref traits, .. }) => traits,
    }
  }

  instance_resolvers: |_| {
    &AltMode => match *self { CharacterMode::AltMode(ref mode) => Some(mode), _ => None },
    &BotMode => match *self { CharacterMode::BotMode(ref mode) => Some(mode), _ => None },
  }
});
