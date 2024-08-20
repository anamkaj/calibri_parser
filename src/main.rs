use calibri_main::calibri_service_data;
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::sync::Arc;
use utils::create_table::create_table;

mod calibri_main;
mod db;
mod models;
mod utils;

pub struct AppState {
    pub db: Pool<Postgres>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let url_connect: String = std::env::var("CALIBRI_TABLE").unwrap();

    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&url_connect)
        .await
    {
        Ok(pool) => {
            println!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    let app_state: Arc<AppState> = Arc::new(AppState { db: pool.clone() });

    // ? Create table
    match create_table(&app_state.db).await {
        Ok(result) => {
            println!("✅ {}", result);
            true
        }
        Err(err) => {
            println!("🔥 Failed to create table: {:?}", err);
            false
        }
    };
    
    let _ = calibri_service_data(3, pool)
        .await
        .expect("Failed to get data");
}
