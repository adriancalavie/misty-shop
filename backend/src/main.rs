mod db;
mod models;
mod services;
mod utils;

use db::supabase::get_db;
use services::*;

async fn list_items() {
    let supabase = get_db();

    let db_lock = supabase.lock().await;
    let all_items = db_lock.list_items().await;

    match all_items {
        Ok(items) => {
            println!("Items are:");
            for item in items {
                println!("{:?}", item);
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}

async fn create_item() {
    println!("Creating item...");
    let supabase = get_db();

    let db_lock = supabase.lock().await;
    let created_item = db_lock.create_item("item_generic").await;

    match created_item {
        Ok(item) => {
            println!("{:?}", item);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}

async fn test_db() {
    println!("Testing db...");
    let supabase = get_db();

    let db_lock = supabase.lock().await;
    let item_by_name = db_lock
        .get_item_by_name("item_generic")
        .await
        .expect("Error getting item named \"item_generic\"")
        .expect("Item named \"item_generic\" doesn't exist");
    let id = item_by_name.id;
    println!("Item by name: {:?}", item_by_name);

    let item_by_id = db_lock
        .get_item_by_id(id)
        .await
        .expect("Error getting item by id")
        .expect("Item with this id doesn't exist");

    println!("Item by id: {:?}", item_by_id);

    let updated_item = db_lock
        .update_item(id, "item_updated")
        .await
        .expect("Error updating item");

    println!("Item updated: {:?}", updated_item);

    let all_items = db_lock
        .list_items()
        .await
        .expect("Error getting list of items");

    println!("Items are:");
    for item in all_items {
        println!("{:?}", item);
    }

    let deleted_item = db_lock.delete_item(id).await;

    println!("Item deleted: {:?}", deleted_item.is_ok());
}

#[tokio::main]
async fn main() {
    list_items().await;
    create_item().await;
    list_items().await;
    test_db().await;
    list_items().await;
}
