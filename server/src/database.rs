use crate::data::CharacterCard;
use crate::data::Image;
use crate::data::StratagemCard;
use crate::data::{BattleCard, BattleCardInput};
use crate::data::{
    BattleType, CardCategory, CardRarity, CharacterMode, CharacterTrait, Faction, ModeType, Node,
    NodeType,
};
use crate::data::{Wave, WaveInput};
use sqlx::postgres::PgPool;
use std::convert::TryFrom;
use uuid::Uuid;

pub type Error = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug, Clone)]
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
        SELECT id, node_id, node_type AS "node_type: NodeType"
        FROM nodes
        WHERE node_type IN ('BATTLE', 'CHARACTER', 'STRATAGEM')
        ORDER BY id;
        "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(card_nodes)
    }

    pub async fn get_node_by_uuid(&self, node_id: Uuid) -> Result<Node, Error> {
        let node = sqlx::query_as!(
            Node,
            r#"
            SELECT id, node_id, node_type AS "node_type: NodeType"
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

// Images
impl Database {
    pub async fn get_image(&self, image_id: i32) -> Result<Image, Error> {
        let image = sqlx::query_as!(
            Image,
            r#"
            SELECT i.id, n.node_id, i.original_url
            FROM images AS i, nodes AS n
            WHERE i.id = n.id AND n.id = $1
            "#,
            image_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(image)
    }
}

// Cards
impl Database {
    pub async fn create_battle_card(&self, input: BattleCardInput) -> Result<BattleCard, Error> {
        let mut tx = self.pool.begin().await?;

        let result = sqlx::query_as!(
            BattleCard,
            r#"
            WITH image_node AS (
                INSERT INTO nodes (node_type) VALUES ('IMAGE') RETURNING *
            ), image AS (
                INSERT INTO images (id, original_url) SELECT n.id, $12 FROM image_node AS n RETURNING *
            ), node AS (
                INSERT INTO nodes (node_type) VALUES ('BATTLE') RETURNING *
            ), wave AS (
                SELECT id FROM waves WHERE tcg_id = $11
            ), card AS (
                INSERT INTO battle_cards (
                    id,
                    category,
                    wave_id,
                    icons,
                    tcg_id,
                    rarity,
                    number,
                    title,
                    type,
                    faction,
                    stars,
                    attack_modifier,
                    defense_modifier,
                    image_id
                ) SELECT
                    n.id,
                    'BATTLE',
                    w.id,
                    CAST($1::TEXT[] as BATTLE_ICON[]),
                    $2,
                    $3,
                    $4,
                    $5,
                    $6,
                    $7,
                    $8,
                    $9,
                    $10,
                    i.id
                FROM node AS n, wave AS w, image AS i RETURNING *
            )
            SELECT
                n.id,
                n.node_id,
                c.tcg_id,
                c.category AS "category: CardCategory",
                c.title,
                c.icons::TEXT[],
                c.type AS "type: BattleType",
                c.rarity AS "rarity: CardRarity",
                c.number,
                c.faction AS "faction: Option<Faction>",
                c.stars,
                c.attack_modifier,
                c.defense_modifier,
                c.image_id
            FROM card AS c, node AS n;
            "#,
            &input.icons,
            input.tcg_id,
            input.rarity as CardRarity,
            input.number,
            input.title,
            input.type_ as BattleType,
            input.faction as Option<Faction>,
            input.stars,
            input.attack_modifier,
            input.defense_modifier,
            input.wave_tcg_id,
            input.image.original_url
        )
        .fetch_one(&mut tx)
        .await;

        match result {
            Ok(wave) => {
                tx.commit().await?;
                Ok(wave)
            }
            Err(err) => {
                println!("{}", err);
                tx.rollback().await?;
                Err(err.into())
            }
        }
    }

    pub async fn get_battle_card(&self, id: i32) -> Result<BattleCard, Error> {
        let battle_card = sqlx::query_as!(
            BattleCard,
            r#"
        SELECT
            bc.id,
            n.node_id,
            bc.tcg_id,
            bc.rarity AS "rarity: CardRarity",
            bc.number,
            bc.category AS "category: CardCategory",
            bc.title,
            bc.icons::TEXT[],
            bc.stars,
            bc.type AS "type: BattleType",
            bc.attack_modifier,
            bc.defense_modifier,
            bc.faction AS "faction: Option<Faction>",
            bc.image_id
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
        SELECT
            cc.id,
            n.node_id,
            cc.tcg_id,
            cc.rarity AS "rarity: CardRarity",
            cc.number,
            cc.category AS "category: CardCategory"
        FROM character_cards AS cc, nodes AS n
        WHERE cc.id = n.id AND n.id = $1;
        "#,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(character_card)
    }

    pub async fn get_stratagem_card(&self, id: i32) -> Result<StratagemCard, Error> {
        let stratagem_card = sqlx::query_as!(
            StratagemCard,
            r#"
        SELECT
            sc.id,
            n.node_id,
            sc.tcg_id,
            sc.rarity AS "rarity: CardRarity",
            sc.number,
            sc.category AS "category: CardCategory",
            sc.title,
            sc.faction AS "faction: Faction",
            sc.requirement,
            sc.stars
        FROM stratagem_cards AS sc, nodes AS n
        WHERE sc.id = n.id AND n.id = $1;
        "#,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(stratagem_card)
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
        SELECT
            cm.id,
            n.node_id,
            cm.title,
            cm.subtitle,
            cm.faction AS "faction: Faction",
            cm.traits::TEXT[],
            cm.type AS "type: ModeType",
            cm.stars,
            cm.health,
            cm.attack,
            cm.defense,
            cm.attack_modifier,
            cm.defense_modifier
        FROM character_modes AS cm, nodes AS n
        WHERE cm.id = n.id AND character_id = $1
        ORDER BY n.id;
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
    pub async fn create_wave(&self, input: WaveInput) -> Result<Wave, Error> {
        let mut tx = self.pool.begin().await?;

        let result = sqlx::query_as!(
            Wave,
            r#"
            WITH node AS (
                INSERT INTO nodes (node_type) VALUES ('WAVE') RETURNING *
            ), wave AS (
                INSERT INTO waves (id, tcg_id, name, released) SELECT id, $1, $2, $3 FROM node RETURNING *
            )
            SELECT n.id, n.node_id, w.tcg_id, w.name, w.released FROM node AS n, wave AS w;
            "#,
            input.tcg_id,
            input.name,
            input.released
        )
        .fetch_one(&mut tx)
        .await;

        match result {
            Ok(wave) => {
                tx.commit().await?;
                Ok(wave)
            }
            Err(err) => {
                println!("{}", err);
                tx.rollback().await?;
                Err(err.into())
            }
        }
    }

    pub async fn get_wave(&self, id: i32) -> Result<Wave, Error> {
        let wave = sqlx::query_as!(
            Wave,
            r#"
            SELECT w.id, n.node_id, w.tcg_id, w.name, w.released
            FROM waves AS w, nodes AS n
            WHERE w.id = n.id AND n.id = $1;
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(wave)
    }

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
