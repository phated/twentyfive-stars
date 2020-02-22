use crate::data::{AltMode, BotMode, CharacterTrait, Faction, ModeType};
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
    // faction,
    Faction,
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
      faction,
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
      ModeType::Alt | ModeType::Alt1 | ModeType::Alt2 => CharacterMode::AltMode(AltMode::new(
        id,
        title,
        subtitle.expect("AltMode must have a subtitle"),
        stars,
        type_,
        faction,
        traits,
        health.expect("AltMode must have health"),
        attack.expect("AltMode must have attack"),
        defense.expect("AltMode must have defense"),
      )),
      ModeType::Bot => CharacterMode::BotMode(BotMode::new(
        id,
        title,
        subtitle.expect("BotMode must have a subtitle"),
        stars,
        type_,
        faction,
        traits,
        health.expect("BotMode must have health"),
        attack.expect("BotMode must have attack"),
        defense.expect("BotMode must have defense"),
      )),
    }
  }
}

impl CharacterMode {
  pub fn id(&self) -> Uuid {
    match self {
      CharacterMode::AltMode(mode) => mode.id(),
      CharacterMode::BotMode(mode) => mode.id(),
    }
  }

  pub fn title(&self) -> &str {
    match self {
      CharacterMode::AltMode(mode) => mode.title(),
      CharacterMode::BotMode(mode) => mode.title(),
    }
  }

  pub fn stars(&self) -> i32 {
    match self {
      CharacterMode::AltMode(mode) => mode.stars(),
      CharacterMode::BotMode(mode) => mode.stars(),
    }
  }

  pub fn type_(&self) -> &ModeType {
    match self {
      CharacterMode::AltMode(mode) => mode.type_(),
      CharacterMode::BotMode(mode) => mode.type_(),
    }
  }

  pub fn faction(&self) -> &Faction {
    match self {
      CharacterMode::AltMode(mode) => mode.faction(),
      CharacterMode::BotMode(mode) => mode.faction(),
    }
  }

  pub fn traits(&self) -> &Vec<CharacterTrait> {
    match self {
      CharacterMode::AltMode(mode) => mode.traits(),
      CharacterMode::BotMode(mode) => mode.traits(),
    }
  }
}

juniper::graphql_interface!(CharacterMode: Context |&self| {
  field id() -> Uuid {
    self.id()
  }

  field title() -> &str {
    self.title()
  }

  field stars() -> i32 {
    self.stars()
  }

  field type_() -> &ModeType {
    self.type_()
  }

  field faction() -> &Faction {
    self.faction()
  }

  field traits() -> &Vec<CharacterTrait> {
    self.traits()
  }

  instance_resolvers: |_| {
    &AltMode => match *self { CharacterMode::AltMode(ref mode) => Some(mode), _ => None },
    &BotMode => match *self { CharacterMode::BotMode(ref mode) => Some(mode), _ => None },
  }
});
