use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Item {
    pub id: i32,
    pub created_at: String,
    pub name: String,
}
