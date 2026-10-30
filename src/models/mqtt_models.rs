use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Command {
    pub id: String,
    pub action: String,
}

