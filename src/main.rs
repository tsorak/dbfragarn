use sqlx::{self, Column, Row};
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
    clear_term();
    println!("Connecting...");
    let pool = sqlx::mysql::MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;
    clear_term();

    let mut query = String::new();
    loop {
        match get_term_input() {
            Ok(s) => {
                query = s;
            }
            Err(e) => eprintln!("Error: {}", e),
        }

        sqlx::query(&query)
            .fetch_all(&pool)
            .await
            .map(|rows| {
                for row in rows {
                    row.columns().iter().for_each(|col| {
                        let i = col.ordinal();
                        let val: String = match row.try_get(i) {
                            Ok(v) => v,
                            Err(_) => match row.try_get::<i32, _>(i) {
                                Ok(v) => v.to_string(),
                                Err(e) => {
                                    dbg!(e);
                                    String::from("(parse error)")
                                }
                            },
                        };
                        println!("{}", val);
                    })
                }
            })
            .unwrap();
    }
}

fn get_term_input() -> Result<String, Box<dyn Error>> {
    use std::io::{stdin, stdout, Write};
    let mut s = String::new();
    print!("dbfragarn> ");
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

fn clear_term() -> () {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}
