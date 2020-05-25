use crate::data::BattleCard;
use crate::data::CharacterCard;
use crate::data::StratagemCard;
use crate::data::{BattleType, CardCategory, CardRarity, Faction, Node, Wave};
use sqlx::postgres::{PgPool, PgQueryAs};

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
}

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

    pub async fn get_card_nodes(&self) -> Result<Vec<Node>, Error> {
        let card_nodes: Vec<Node> = sqlx::query_as(
            "SELECT id, node_id, node_type, has_previous, has_next FROM cards_with_pageinfo",
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(card_nodes)
    }
}
