use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct Item {
    pub id: Uuid,
    pub created_at: String,
    pub name: String,
}
