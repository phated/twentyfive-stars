use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    permissions: Vec<String>,
}

impl User {
    pub fn empty() -> Self {
        User {
            permissions: Vec::with_capacity(0),
        }
    }
}
