use crate::data::{AltMode, BotMode, CharacterTrait, ModeType};
use crate::database_schema::character_modes;
use crate::graphql_schema::Context;
use diesel::deserialize::Queryable;
use uuid::Uuid;

type DB = diesel::pg::Pg;

pub enum CharacterMode {
  AltMode(AltMode),
  BotMode(BotMode),
  // CombinerMode(CombinerMode),
  // BodyMode(BodyMode),
  // HeadMode(HeadMode),
  // UpgradeMode(UpgradeMode),
}

impl Queryable<character_modes::SqlType, DB> for CharacterMode {
  type Row = (
    // id
    Uuid,
    // card_id
    Uuid,
    // title
    String,
    // subtitle
    Option<String>,
    // traits
    Vec<CharacterTrait>,
    // type
    ModeType,
    // stars
    i32,
    // health
    Option<i32>,
    // attack
    Option<i32>,
    // defense
    Option<i32>,
    // attack_modifier
    Option<i32>,
    // defense_modifier
    Option<i32>,
  );

  fn build(row: Self::Row) -> Self {
    let (
      id,
      _card_id,
      title,
      subtitle,
      traits,
      type_,
      stars,
      health,
      attack,
      defense,
      _attack_modifier,
      _defense_modifier,
    ) = row;

    match type_ {
      ModeType::Alt | ModeType::Alt1 | ModeType::Alt2 => CharacterMode::AltMode(AltMode {
        id,
        title,
        subtitle: subtitle.expect("AltMode must have a subtitle"),
        stars,
        type_,
        traits,
        health: health.expect("AltMode must have health"),
        attack: attack.expect("AltMode must have attack"),
        defense: defense.expect("AltMode must have defense"),
      }),
      ModeType::Bot => CharacterMode::BotMode(BotMode {
        id,
        title,
        subtitle: subtitle.expect("BotMode must have a subtitle"),
        stars,
        type_,
        traits,
        health: health.expect("BotMode must have health"),
        attack: attack.expect("BotMode must have attack"),
        defense: defense.expect("BotMode must have defense"),
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
