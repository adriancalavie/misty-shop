use axum::async_trait;
use serde::de::DeserializeOwned;
use serde_json::json;
use uuid::Uuid;

pub use crate::db::supabase::Supabase;
use crate::json_string;
use crate::models::Item;

use super::DataLayer;
use super::Result;

/// Response body by default is a JSON array and we only want the first(and the only) element
async fn handle_response<T: DeserializeOwned>(
    resp: reqwest::Result<reqwest::Response>,
) -> Result<T> {
    match resp {
        Ok(resp) => {
            let mut json_array = resp.json::<Vec<T>>().await?;
            let json = json_array.pop().expect("Response body empty");
            Ok(json)
        }
        Err(e) => Err(e.into()),
    }
}

#[async_trait]
impl DataLayer for Supabase {
    async fn create_item<'a>(&self, name: &'a str) -> Result<Item> {
        let resp: std::prelude::v1::Result<reqwest::Response, reqwest::Error> = self
            .from("items")
            .insert(json_string!({"name": name}))
            .execute()
            .await;
        handle_response(resp).await
    }
    async fn get_item_by_id(&self, id: Uuid) -> Result<Option<Item>> {
        let resp = self.from("items").eq("id", id.to_string()).execute().await;
        handle_response(resp).await
    }
    async fn get_item_by_name<'a>(&self, name: &'a str) -> Result<Option<Item>> {
        let resp = self.from("items").eq("name", name).execute().await;
        handle_response(resp).await
    }
    async fn update_item<'a>(&self, id: Uuid, name: &'a str) -> Result<Item> {
        let resp = self
            .from("items")
            .eq("id", id.to_string())
            .update(json_string!({"name": name}))
            .execute()
            .await;
        handle_response(resp).await
    }
    async fn delete_item(&self, id: Uuid) -> Result<()> {
        let resp = self
            .from("items")
            .eq("id", id.to_string())
            .delete()
            .execute()
            .await;
        match resp {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }

    async fn list_items(&self) -> Result<Vec<Item>> {
        let resp = self.from("items").execute().await;
        match resp {
            Ok(resp) => {
                let items = resp.json::<Vec<Item>>().await?;
                Ok(items)
            }
            Err(e) => Err(e.into()),
        }
    }
}
