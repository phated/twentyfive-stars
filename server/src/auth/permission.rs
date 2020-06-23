use crate::user::User;
use async_graphql::guard::Guard;
use async_graphql::{Context, FieldResult};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, PartialEq)]
pub enum Permission {
    CreateWaves,
    CreateBattleCards,
    Other(String),
}

impl Serialize for Permission {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(match *self {
            Permission::CreateWaves => "create:waves",
            Permission::CreateBattleCards => "create:battle_cards",
            Permission::Other(ref other) => other,
        })
    }
}

impl<'de> Deserialize<'de> for Permission {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "create:waves" => Permission::CreateWaves,
            "create:battle_cards" => Permission::CreateBattleCards,
            _ => Permission::Other(s),
        })
    }
}

pub struct PermissionGuard {
    pub permission: Permission,
}

#[async_trait::async_trait]
impl Guard for PermissionGuard {
    async fn check(&self, ctx: &Context<'_>) -> FieldResult<()> {
        let maybe_user = ctx.data_opt::<User>();
        if let Some(user) = maybe_user {
            if user.has_permission(&self.permission) {
                Ok(())
            } else {
                Err("Invalid permissions".into())
            }
        } else {
            Err("Must be authenticated".into())
        }
    }
}
