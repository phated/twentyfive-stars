use crate::auth::Permission;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    permissions: Vec<Permission>,
}

impl User {
    pub fn empty() -> Self {
        User {
            permissions: Vec::with_capacity(0),
        }
    }
}

impl User {
    pub fn has_permission(&self, permission: &Permission) -> bool {
        self.permissions.contains(permission)
    }
}
