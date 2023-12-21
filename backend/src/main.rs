mod models;
use models::Item;

mod db;
use db::supabase::get_db;

use reqwest::Response;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let supabase = get_db();

    let db_lock = supabase.lock().await;
    let resp: Response = db_lock.get_all_items().await?;

    if !resp.status().is_success() {
        eprintln!("Error: {}", resp.text().await?);
    } else {
        let objects: Vec<Item> = resp.json().await?;

        for obj in objects {
            println!("{:?}", obj);
        }
    }

    Ok(())
}
