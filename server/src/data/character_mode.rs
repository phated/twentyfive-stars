use crate::data::{CharacterTrait, Faction, ModeType};
use async_graphql::{Interface, ID};
use uuid::Uuid;

pub mod alt_mode;
pub mod body_mode;
pub mod bot_mode;
pub mod combiner_body_mode;
pub mod combiner_mode;
pub mod head_mode;
pub mod upgrade_mode;

pub use alt_mode::AltMode;
pub use body_mode::BodyMode;
pub use bot_mode::BotMode;
pub use combiner_body_mode::CombinerBodyMode;
pub use combiner_mode::CombinerMode;
pub use head_mode::HeadMode;
pub use upgrade_mode::UpgradeMode;

#[derive(Interface, Clone, Debug)]
#[graphql(
    field(name = "id", type = "ID"),
    field(name = "title", type = "&str"),
    field(name = "stars", type = "i32"),
    field(name = "type_", type = "ModeType"),
    field(name = "faction", type = "Faction")
)]
pub enum CharacterMode {
    AltMode(AltMode),
    BotMode(BotMode),
    CombinerMode(CombinerMode),
    UpgradeMode(UpgradeMode),
    BodyMode(BodyMode),
    HeadMode(HeadMode),
    CombinerBodyMode(CombinerBodyMode),
}

impl CharacterMode {
    pub fn new(
        id: i32,
        node_id: Uuid,
        title: String,
        subtitle: Option<String>,
        faction: Faction,
        traits: Vec<CharacterTrait>,
        type_: ModeType,
        stars: i32,
        health: Option<i32>,
        attack: Option<i32>,
        defense: Option<i32>,
        attack_modifier: Option<i32>,
        defense_modifier: Option<i32>,
    ) -> Self {
        match type_ {
            ModeType::Alt | ModeType::Alt1 | ModeType::Alt2 => CharacterMode::AltMode(AltMode {
                node_id,
                id,
                title,
                subtitle: subtitle.expect("AltMode must have a subtitle"),
                stars,
                type_,
                faction,
                traits,
                health: health.expect("AltMode must have health"),
                attack: attack.expect("AltMode must have attack"),
                defense: defense.expect("AltMode must have defense"),
            }),
            ModeType::Bot => CharacterMode::BotMode(BotMode {
                node_id,
                id,
                title,
                subtitle: subtitle.expect("BotMode must have a subtitle"),
                stars,
                type_,
                faction,
                traits,
                health: health.expect("BotMode must have health"),
                attack: attack.expect("BotMode must have attack"),
                defense: defense.expect("BotMode must have defense"),
            }),
            ModeType::Combiner => CharacterMode::CombinerMode(CombinerMode {
                node_id,
                id,
                title,
                subtitle: subtitle.expect("CombinerMode must have a subtitle"),
                stars,
                type_,
                faction,
                traits,
                health: health.expect("CombinerMode must have health"),
                attack: attack.expect("CombinerMode must have attack"),
                defense: defense.expect("CombinerMode must have defense"),
            }),
            ModeType::UpgradeWeapon | ModeType::UpgradeArmor | ModeType::UpgradeUtility => {
                CharacterMode::UpgradeMode(UpgradeMode {
                    node_id,
                    id,
                    title,
                    stars,
                    type_,
                    faction,
                    traits,
                    attack_modifier,
                    defense_modifier,
                })
            }
            ModeType::Body => CharacterMode::BodyMode(BodyMode {
                node_id,
                id,
                title,
                subtitle: subtitle.expect("BodyMode must have a subtitle"),
                stars,
                type_,
                faction,
                traits,
                health: health.expect("BodyMode must have health"),
                attack: attack.expect("BodyMode must have attack"),
                defense: defense.expect("BodyMode must have defense"),
            }),
            ModeType::Head => CharacterMode::HeadMode(HeadMode {
                node_id,
                id,
                title,
                stars,
                type_,
                faction,
            }),
            ModeType::CombinerBody => CharacterMode::CombinerBodyMode(CombinerBodyMode {
                node_id,
                id,
                title,
                subtitle: subtitle.expect("CombinerBodyMode must have a subtitle"),
                stars,
                type_,
                faction,
                traits,
                health: health.expect("CombinerBodyMode must have health"),
                attack: attack.expect("CombinerBodyMode must have attack"),
                defense: defense.expect("CombinerBodyMode must have defense"),
            }),
        }
    }
}
