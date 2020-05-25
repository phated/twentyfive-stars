#[async_graphql::Enum]
#[derive(Debug, Clone, sqlx::Type)]
#[sqlx(rename = "CHARACTER_TRAIT")]
#[sqlx(rename_all = "uppercase")]
pub enum CharacterTrait {
    Melee,
    Ranged,
    Specialist,
    Motorcycle,
    Spaceship,
    Truck,
    Leader,
    Car,
    Insecticon,
    Tank,
    Dinobot,
    Plane,
}
