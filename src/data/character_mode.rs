use crate::data::{
  AltMode, BodyMode, BotMode, CharacterTrait, CombinerBodyMode, CombinerMode, Faction, HeadMode,
  ModeType, UpgradeMode,
};
use crate::database_schema::character_modes;
use crate::graphql_schema::Context;
use diesel::deserialize::Queryable;
use uuid::Uuid;

type DB = diesel::pg::Pg;

pub enum CharacterMode {
  AltMode(AltMode),
  BotMode(BotMode),
  CombinerMode(CombinerMode),
  UpgradeMode(UpgradeMode),
  BodyMode(BodyMode),
  HeadMode(HeadMode),
  CombinerBodyMode(CombinerBodyMode),
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
      attack_modifier,
      defense_modifier,
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
      ModeType::Combiner => CharacterMode::CombinerMode(CombinerMode::new(
        id,
        title,
        subtitle.expect("CombinerMode must have a subtitle"),
        stars,
        type_,
        faction,
        traits,
        health.expect("CombinerMode must have health"),
        attack.expect("CombinerMode must have attack"),
        defense.expect("CombinerMode must have defense"),
      )),
      ModeType::UpgradeWeapon | ModeType::UpgradeArmor | ModeType::UpgradeUtility => {
        CharacterMode::UpgradeMode(UpgradeMode::new(
          id,
          title,
          stars,
          type_,
          faction,
          traits,
          attack_modifier,
          defense_modifier,
        ))
      }
      ModeType::Body => CharacterMode::BodyMode(BodyMode::new(
        id,
        title,
        subtitle.expect("BodyMode must have a subtitle"),
        stars,
        type_,
        faction,
        traits,
        health.expect("BodyMode must have health"),
        attack.expect("BodyMode must have attack"),
        defense.expect("BodyMode must have defense"),
      )),
      ModeType::Head => CharacterMode::HeadMode(HeadMode::new(id, title, stars, type_, faction)),
      ModeType::CombinerBody => CharacterMode::CombinerBodyMode(CombinerBodyMode::new(
        id,
        title,
        subtitle.expect("CombinerBodyMode must have a subtitle"),
        stars,
        type_,
        faction,
        traits,
        health.expect("CombinerBodyMode must have health"),
        attack.expect("CombinerBodyMode must have attack"),
        defense.expect("CombinerBodyMode must have defense"),
      )),
    }
  }
}

impl CharacterMode {
  pub fn id(&self) -> Uuid {
    match self {
      CharacterMode::AltMode(mode) => mode.id(),
      CharacterMode::BotMode(mode) => mode.id(),
      CharacterMode::CombinerMode(mode) => mode.id(),
      CharacterMode::UpgradeMode(mode) => mode.id(),
      CharacterMode::BodyMode(mode) => mode.id(),
      CharacterMode::HeadMode(mode) => mode.id(),
      CharacterMode::CombinerBodyMode(mode) => mode.id(),
    }
  }

  pub fn title(&self) -> &str {
    match self {
      CharacterMode::AltMode(mode) => mode.title(),
      CharacterMode::BotMode(mode) => mode.title(),
      CharacterMode::CombinerMode(mode) => mode.title(),
      CharacterMode::UpgradeMode(mode) => mode.title(),
      CharacterMode::BodyMode(mode) => mode.title(),
      CharacterMode::HeadMode(mode) => mode.title(),
      CharacterMode::CombinerBodyMode(mode) => mode.title(),
    }
  }

  pub fn stars(&self) -> i32 {
    match self {
      CharacterMode::AltMode(mode) => mode.stars(),
      CharacterMode::BotMode(mode) => mode.stars(),
      CharacterMode::CombinerMode(mode) => mode.stars(),
      CharacterMode::UpgradeMode(mode) => mode.stars(),
      CharacterMode::BodyMode(mode) => mode.stars(),
      CharacterMode::HeadMode(mode) => mode.stars(),
      CharacterMode::CombinerBodyMode(mode) => mode.stars(),
    }
  }

  pub fn type_(&self) -> &ModeType {
    match self {
      CharacterMode::AltMode(mode) => mode.type_(),
      CharacterMode::BotMode(mode) => mode.type_(),
      CharacterMode::CombinerMode(mode) => mode.type_(),
      CharacterMode::UpgradeMode(mode) => mode.type_(),
      CharacterMode::BodyMode(mode) => mode.type_(),
      CharacterMode::HeadMode(mode) => mode.type_(),
      CharacterMode::CombinerBodyMode(mode) => mode.type_(),
    }
  }

  pub fn faction(&self) -> &Faction {
    match self {
      CharacterMode::AltMode(mode) => mode.faction(),
      CharacterMode::BotMode(mode) => mode.faction(),
      CharacterMode::CombinerMode(mode) => mode.faction(),
      CharacterMode::UpgradeMode(mode) => mode.faction(),
      CharacterMode::BodyMode(mode) => mode.faction(),
      CharacterMode::HeadMode(mode) => mode.faction(),
      CharacterMode::CombinerBodyMode(mode) => mode.faction(),
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

  instance_resolvers: |_| {
    &AltMode => match *self { CharacterMode::AltMode(ref mode) => Some(mode), _ => None },
    &BotMode => match *self { CharacterMode::BotMode(ref mode) => Some(mode), _ => None },
    &CombinerMode => match *self { CharacterMode::CombinerMode(ref mode) => Some(mode), _ => None },
    &UpgradeMode => match *self { CharacterMode::UpgradeMode(ref mode) => Some(mode), _ => None },
    &BodyMode => match *self { CharacterMode::BodyMode(ref mode) => Some(mode), _ => None },
    &HeadMode => match *self { CharacterMode::HeadMode(ref mode) => Some(mode), _ => None },
    &CombinerBodyMode => match *self { CharacterMode::CombinerBodyMode(ref mode) => Some(mode), _ => None },
  }
});
