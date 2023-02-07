use sqlx::mysql::MySqlPoolOptions;
use std::{env, process};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let db_url = match env::var("URL") {
        Ok(v) => String::from(v),
        Err(_) => {
            println!("Please assign a URL environment variable. Template URL: 'mysql://USERNAME:PASSWORD@HOST/DATABASE'");
            process::exit(1);
        }
    };
    // Create a connection pool
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    // Make a simple query to return the given parameter
    let row: (i64,) = sqlx::query_as("SELECT ?")
        .bind(150_i64)
        .fetch_one(&pool)
        .await?;

    assert_eq!(row.0, 150);

    Ok(())
}
