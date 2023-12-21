
use dotenv::dotenv;
use postgrest::Postgrest;
use reqwest::Response;
use tokio::sync::Mutex;

pub struct Supabase {
    pub client: Postgrest,
}

impl Supabase {
    fn new() -> Self {
        dotenv().ok(); // Load the environment variables from the .env file
        let supabase_anon_key =
            std::env::var("SUPABASE_ANON_KEY").expect("SUPABASE_ANON_KEY not set in .env");

        Self {
            client: Postgrest::new("https://hvutwgemvirmzqqycerg.supabase.co/rest/v1")
                .insert_header("apikey", supabase_anon_key),
        }
    }

    pub async fn get_all_items(&self) -> Result<Response, reqwest::Error> {
        self.client.from("items").select("*").execute().await
    }
}

lazy_static::lazy_static! {
    static ref SUPABASE_INSTANCE: Mutex<Supabase> = Mutex::new(Supabase::new());
}

pub fn get_db() -> &'static Mutex<Supabase> {
    &SUPABASE_INSTANCE
}
