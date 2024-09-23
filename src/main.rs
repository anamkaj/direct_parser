use dotenv::dotenv;

use models::client_stat::req_stat_client;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::sync::Arc;
use utils::create_table::create_table;

mod db;
mod models;
mod utils;

pub struct AppState {
    pub db_direct: Pool<Postgres>,
    pub db_status: Pool<Postgres>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let url_connect: String = std::env::var("DIRECT_TABLE").unwrap();
    let status_table: String = std::env::var("STATUS_TABLE").unwrap();

    let pool: Pool<Postgres> = match PgPoolOptions::new()
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

    let pool_status: Pool<Postgres> = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&status_table)
        .await
    {
        Ok(pool) => {
            println!("✅Connection to the database pool_status is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };
    //Подключение к БД direct и status_table
    let state: Arc<AppState> = Arc::new(AppState {
        db_direct: pool, // direct
        db_status: pool_status,
    });

    // ? Create table
    match create_table(&state.db_status).await {
        Ok(result) => {
            println!("✅ {}", result);
            true
        }
        Err(err) => {
            println!("🔥 Failed to create table: {:?}", err);
            false
        }
    };

    let result: String = start_parse_direct(&state).await;
    println!("{}", result);
}

async fn start_parse_direct(state: &Arc<AppState>) -> String {
    let q: &str = "INSERT INTO status_table (status) VALUES($1);";

    let result: String = match req_stat_client(state.db_direct.clone()).await {
        Ok(_) => {
            let status: String = "Service Direct: Parse successful".to_string();

            sqlx::query(&q)
                .bind(&status)
                .execute(&state.db_status)
                .await
                .unwrap();

            status
        }

        Err(err) => {
            let status: String = format!("Service Direct: Error parsing {:?}", err);

            sqlx::query(&q)
                .bind(&status)
                .execute(&state.db_status)
                .await
                .unwrap();

            status
        }
    };

    result
}
