use crate::data::BattleCard;
use crate::data::CharacterCard;
use crate::data::StratagemCard;
use crate::data::{
    BattleType, CardCategory, CardRarity, CharacterMode, CharacterTrait, Faction, ModeType, Node,
    NodeType, Wave,
};
use sqlx::postgres::PgPool;
use std::convert::TryFrom;
use uuid::Uuid;

pub type Error = Box<dyn std::error::Error + Send + Sync>;

pub struct Database {
    pub pool: PgPool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Database, Error> {
        let pool = PgPool::builder()
            .max_size(5) // maximum number of connections in the pool
            .build(database_url)
            .await?;
        let db = Database { pool };
        Ok(db)
    }
}

// Nodes
impl Database {
    pub async fn get_card_nodes(&self) -> Result<Vec<Node>, Error> {
        let card_nodes = sqlx::query_as!(
            Node,
            r#"
            SELECT id, node_id, node_type
            FROM nodes
            WHERE node_type IN ('BATTLE', 'CHARACTER', 'STRATAGEM')"#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(card_nodes)
    }

    pub async fn get_node_by_uuid(&self, node_id: Uuid) -> Result<Node, Error> {
        let node = sqlx::query_as!(
            Node,
            r#"
            SELECT id, node_id, node_type
            FROM nodes
            WHERE node_id = $1;
            "#,
            node_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(node)
    }
}

// Cards
impl Database {
    pub async fn get_battle_card(&self, id: i32) -> Result<BattleCard, Error> {
        let battle_card = sqlx::query_as!(
            BattleCard,
            r#"
        SELECT bc.id, n.node_id, bc.tcg_id, bc.rarity, bc.number, bc.category, bc.title, bc.stars, bc.type, bc.attack_modifier, bc.defense_modifier, bc.faction
        FROM battle_cards AS bc, nodes AS n
        WHERE bc.id = n.id AND n.id = $1;
        "#,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(battle_card)
    }

    pub async fn get_character_card(&self, id: i32) -> Result<CharacterCard, Error> {
        let character_card = sqlx::query_as!(
            CharacterCard,
            r#"
        SELECT cc.id, n.node_id, cc.tcg_id, cc.rarity, cc.number, cc.category
        FROM character_cards AS cc, nodes AS n
        WHERE cc.id = n.id AND n.id = $1;
        "#,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(character_card)
    }
}

// Character Modes
impl Database {
    pub async fn get_modes_for_character_card(
        &self,
        card: &CharacterCard,
    ) -> Result<Vec<CharacterMode>, Error> {
        let rows = sqlx::query!(
            r#"
        SELECT cm.id, n.node_id, cm.title, cm.subtitle, cm.faction, cm.traits::TEXT[], cm.type,
        cm.stars, cm.health, cm.attack, cm.defense, cm.attack_modifier, cm.defense_modifier
        FROM character_modes AS cm, nodes AS n
        WHERE cm.id = n.id AND character_id = $1
        "#,
            card.id
        )
        .fetch_all(&self.pool)
        .await?;

        let mut modes = vec![];
        for row in rows.iter() {
            let mut traits = vec![];
            let trait_values = row.traits.clone().expect("Character mode must have traits");
            for trait_value in trait_values {
                traits.push(CharacterTrait::try_from(trait_value)?);
            }

            let mode = CharacterMode::new(
                row.id,
                row.node_id.expect("Character mode must have node_id"),
                row.title.clone(),
                row.subtitle.clone(),
                row.faction,
                traits,
                row.r#type,
                row.stars,
                row.health,
                row.attack,
                row.defense,
                row.attack_modifier,
                row.defense_modifier,
            );

            modes.push(mode);
        }

        Ok(modes)
    }
}

// Waves
impl Database {
    pub async fn get_wave_for_battle_card(&self, card: &BattleCard) -> Result<Wave, Error> {
        let wave = sqlx::query_as!(
            Wave,
            r#"
            SELECT w.id, n.node_id, w.tcg_id, w.name, w.released
            FROM battle_cards AS bc, waves AS w, nodes AS n
            WHERE w.id = n.id AND w.id = bc.wave_id AND bc.id = $1;
            "#,
            card.id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(wave)
    }

    pub async fn get_wave_for_character_card(&self, card: &CharacterCard) -> Result<Wave, Error> {
        let wave = sqlx::query_as!(
            Wave,
            r#"
            SELECT w.id, n.node_id, w.tcg_id, w.name, w.released
            FROM character_cards AS c, waves AS w, nodes AS n
            WHERE w.id = n.id AND w.id = c.wave_id AND c.id = $1;
            "#,
            card.id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(wave)
    }

    pub async fn get_wave_for_stratagem_card(&self, card: &StratagemCard) -> Result<Wave, Error> {
        let wave = sqlx::query_as!(
            Wave,
            r#"
            SELECT w.id, n.node_id, w.tcg_id, w.name, w.released
            FROM stratagem_cards AS s, waves AS w, nodes AS n
            WHERE w.id = n.id AND w.id = s.wave_id AND s.id = $1;
            "#,
            card.id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(wave)
    }
}
