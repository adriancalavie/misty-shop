use dotenv::dotenv;
use postgrest::Postgrest;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Item {
    id: i32,
    created_at: String,
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok(); // Load the environment variables from the .env file

    let supabase_anon_key =
        std::env::var("SUPABASE_ANON_KEY").expect("SUPABASE_ANON_KEY not set in .env");

    let client = Postgrest::new("https://hvutwgemvirmzqqycerg.supabase.co/rest/v1")
        .insert_header("apikey", supabase_anon_key);

    let resp = client.from("items").select("*").execute().await?;
    // println!("Response: {:?}", resp);

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
