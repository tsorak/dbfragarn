use sqlx::mysql::MySqlPoolOptions;
use std::{env, error::Error, process};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let db_url = match env::var("URL") {
        Ok(v) => String::from(v),
        Err(_) => {
            println!("Please assign a URL environment variable.");
            println!("Template URL: 'mysql://USERNAME:PASSWORD@HOST/DATABASE'");
            process::exit(1);
        }
    };
    // Create a connection pool
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    let mut query = String::new();
    loop {
        match get_term_input() {
            Ok(s) => {
                query = s;
            }
            Err(e) => eprintln!("Error: {}", e),
        }

        let row = sqlx::query(&query)
            // .bind(150_i64)
            .fetch_one(&pool)
            .await?;

        println!("{:#?}", row);
    }
}

fn get_term_input() -> Result<String, Box<dyn Error>> {
    use std::io::{stdin, stdout, Write};
    let mut s = String::new();
    print!("Please enter some text: ");
    let _ = stdout().flush();
    match stdin().read_line(&mut s) {
        Ok(_) => {
            if let Some('\n') = s.chars().next_back() {
                s.pop();
            }
            if let Some('\r') = s.chars().next_back() {
                s.pop();
            }
        }
        Err(_) => (),
    }

    Ok(s)
}
