use uuid::Uuid;

#[derive(Debug, Copy, Clone, sqlx::Type)]
#[sqlx(rename = "NODE_TYPE")]
#[sqlx(rename_all = "uppercase")]
pub enum NodeType {
    Character,
    Battle,
    Stratagem,
    Wave,
    #[sqlx(rename = "CHARACTER_MODE")]
    CharacterMode,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Node {
    pub id: i32,
    pub node_id: Uuid,
    pub node_type: NodeType,
    pub has_previous: bool,
    pub has_next: bool,
}
