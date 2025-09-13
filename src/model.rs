use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Item {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}

#[derive(Deserialize, Clone)]
pub struct JsonItem {
    pub name: String,
    pub description: String,
}
