use std::convert::TryFrom;

#[async_graphql::Enum]
#[derive(Debug, Clone)]
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

// TODO: Remove this once sqlx can support arrays of custom types
impl TryFrom<String> for CharacterTrait {
    type Error = Box<dyn std::error::Error + Send + Sync>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "MELEE" => Ok(CharacterTrait::Melee),
            "RANGED" => Ok(CharacterTrait::Ranged),
            "SPECIALIST" => Ok(CharacterTrait::Specialist),
            "MOTORCYCLE" => Ok(CharacterTrait::Motorcycle),
            "SPACESHIP" => Ok(CharacterTrait::Spaceship),
            "TRUCK" => Ok(CharacterTrait::Truck),
            "LEADER" => Ok(CharacterTrait::Leader),
            "CAR" => Ok(CharacterTrait::Car),
            "INSECTICON" => Ok(CharacterTrait::Insecticon),
            "TANK" => Ok(CharacterTrait::Tank),
            "DINOBOT" => Ok(CharacterTrait::Dinobot),
            "PLANE" => Ok(CharacterTrait::Plane),
            val => {
                let msg = format!("Invalid character trait: {}", val);
                Err(msg.into())
            }
        }
    }
}
