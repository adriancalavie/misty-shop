use axum::{async_trait, BoxError};
use uuid::Uuid;

use crate::models::Item;

pub type Result<T> = std::result::Result<T, BoxError>;

#[async_trait]
pub trait DataLayer: Send + Sync {
    async fn create_item<'a>(&self, name: &'a str) -> Result<Item>;
    async fn get_item_by_id(&self, id: Uuid) -> Result<Option<Item>>;
    async fn get_item_by_name<'a>(&self, name: &'a str) -> Result<Option<Item>>;
    async fn update_item<'a>(&self, id: Uuid, name: &'a str) -> Result<Item>;
    async fn delete_item(&self, id: Uuid) -> Result<()>;
    async fn list_items(&self) -> Result<Vec<Item>>;
}
