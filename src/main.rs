use sqlx::{self, Column, Row};
use std::{collections::HashMap, env, error::Error, process};

#[derive(Debug)]
struct Tablecol {
    name: String,
    display_width: i8,
}

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

        match sqlx::query(&query).fetch_all(&pool).await.map(|rows| {
            //Table header
            let mut header_data: HashMap<String, Tablecol> = HashMap::new();
            rows.first()
                .unwrap()
                .columns()
                .iter()
                .for_each(|header_col| {
                    header_data.insert(
                        header_col.ordinal().to_string(),
                        Tablecol {
                            name: (header_col.name().to_owned()),
                            display_width: (header_col.name().chars().count() as i8),
                        },
                    );
                });

            //Table body
            let mut body_data: Vec<Vec<String>> = Vec::new();
            for row in &rows {
                let mut row_data: Vec<String> = Vec::new();
                row.columns().iter().for_each(|col| {
                    let col_id = col.ordinal();
                    let val: String = match row.try_get(col_id) {
                        Ok(v) => v,
                        Err(_) => match row.try_get::<i32, _>(col_id) {
                            Ok(v) => v.to_string(),
                            Err(_e) => {
                                // dbg!(e);
                                String::from("(parse error)")
                            }
                        },
                    };

                    row_data.push(val.clone());
                    // print!("{} ", val);
                    //increase column display_width if needed
                    if val.chars().count() as i8
                        > header_data.get(&col_id.to_string()).unwrap().display_width
                    {
                        header_data.entry(col_id.to_string()).and_modify(|e| {
                            e.display_width = val.chars().count() as i8;
                        });
                    }
                });
                body_data.push(row_data);
            }

            //render
            draw_header(&header_data);
            for row in body_data {
                draw_body_row(&row, &header_data);
                print!("\n");
            }
        }) {
            Ok(_) => (),
            Err(_) => (),
        };
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

fn draw_header(titles: &HashMap<String, Tablecol>) -> () {
    print!("|");
    let mut col_id: u8 = 0;
    for _ in titles {
        // let (k, col) = title;
        let data = titles.get(&col_id.to_string()).unwrap();
        print!(" {} ", data.name);

        let padding: i8 = data.display_width - data.name.chars().count() as i8;
        let mut i: i8 = 0;
        while i < padding {
            print!(" ");
            i = i + 1;
        }

        print!("|");
        col_id = col_id + 1;
    }
    print!("\n");
}

fn draw_body_row(row: &Vec<String>, table_data: &HashMap<String, Tablecol>) -> () {
    print!("|");
    let mut col_id: u8 = 0;
    for val in row {
        print!(" {} ", val);

        let padding: i8 =
            table_data.get(&col_id.to_string()).unwrap().display_width - val.chars().count() as i8;
        let mut i: i8 = 0;
        while i < padding {
            print!(" ");
            i = i + 1;
        }

        print!("|");

        col_id = col_id + 1;
    }
}
