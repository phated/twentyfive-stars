use crate::data::{
  AltMode, BodyMode, BotMode, CharacterTrait, CombinerBodyMode, CombinerMode, Faction, HeadMode,
  ModeType, UpgradeMode, ID,
};
use crate::database_schema::character_modes;
use diesel::deserialize::Queryable;

type DB = diesel::pg::Pg;

#[async_graphql::Interface(
  // field(name = "id", type = "ID"),
  field(name = "title", type = "String"),
  field(name = "stars", type = "i32"),
  field(name = "type_", type = "ModeType"),
  field(name = "faction", type = "Faction")
)]
#[derive(Clone, Debug)]
pub struct CharacterMode(
  AltMode,
  BotMode,
  CombinerMode,
  UpgradeMode,
  BodyMode,
  HeadMode,
  CombinerBodyMode,
);

// impl Queryable<character_modes::SqlType, DB> for CharacterMode {
//   type Row = (
//     // id
//     ID,
//     // card_id
//     ID,
//     // title
//     String,
//     // subtitle
//     Option<String>,
//     // faction,
//     Faction,
//     // traits
//     Vec<CharacterTrait>,
//     // type
//     ModeType,
//     // stars
//     i32,
//     // health
//     Option<i32>,
//     // attack
//     Option<i32>,
//     // defense
//     Option<i32>,
//     // attack_modifier
//     Option<i32>,
//     // defense_modifier
//     Option<i32>,
//     // sort order
//     i32,
//   );

//   fn build(row: Self::Row) -> Self {
//     let (
//       id,
//       _card_id,
//       title,
//       subtitle,
//       faction,
//       traits,
//       type_,
//       stars,
//       health,
//       attack,
//       defense,
//       attack_modifier,
//       defense_modifier,
//       _sort_order,
//     ) = row;

//     match type_ {
//       ModeType::Alt | ModeType::Alt1 | ModeType::Alt2 => CharacterMode::AltMode(AltMode {
//         id,
//         title,
//         subtitle: subtitle.expect("AltMode must have a subtitle"),
//         stars,
//         type_,
//         faction,
//         traits,
//         health: health.expect("AltMode must have health"),
//         attack: attack.expect("AltMode must have attack"),
//         defense: defense.expect("AltMode must have defense"),
//       }),
//       ModeType::Bot => CharacterMode::BotMode(BotMode {
//         id,
//         title,
//         subtitle: subtitle.expect("BotMode must have a subtitle"),
//         stars,
//         type_,
//         faction,
//         traits,
//         health: health.expect("BotMode must have health"),
//         attack: attack.expect("BotMode must have attack"),
//         defense: defense.expect("BotMode must have defense"),
//       }),
//       ModeType::Combiner => CharacterMode::CombinerMode(CombinerMode {
//         id,
//         title,
//         subtitle: subtitle.expect("CombinerMode must have a subtitle"),
//         stars,
//         type_,
//         faction,
//         traits,
//         health: health.expect("CombinerMode must have health"),
//         attack: attack.expect("CombinerMode must have attack"),
//         defense: defense.expect("CombinerMode must have defense"),
//       }),
//       ModeType::UpgradeWeapon | ModeType::UpgradeArmor | ModeType::UpgradeUtility => {
//         CharacterMode::UpgradeMode(UpgradeMode {
//           id,
//           title,
//           stars,
//           type_,
//           faction,
//           traits,
//           attack_modifier,
//           defense_modifier,
//         })
//       }
//       ModeType::Body => CharacterMode::BodyMode(BodyMode {
//         id,
//         title,
//         subtitle: subtitle.expect("BodyMode must have a subtitle"),
//         stars,
//         type_,
//         faction,
//         traits,
//         health: health.expect("BodyMode must have health"),
//         attack: attack.expect("BodyMode must have attack"),
//         defense: defense.expect("BodyMode must have defense"),
//       }),
//       ModeType::Head => CharacterMode::HeadMode(HeadMode {
//         id,
//         title,
//         stars,
//         type_,
//         faction,
//       }),
//       ModeType::CombinerBody => CharacterMode::CombinerBodyMode(CombinerBodyMode {
//         id,
//         title,
//         subtitle: subtitle.expect("CombinerBodyMode must have a subtitle"),
//         stars,
//         type_,
//         faction,
//         traits,
//         health: health.expect("CombinerBodyMode must have health"),
//         attack: attack.expect("CombinerBodyMode must have attack"),
//         defense: defense.expect("CombinerBodyMode must have defense"),
//       }),
//     }
//   }
// }
