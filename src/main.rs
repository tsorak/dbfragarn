use sqlx::{self, mysql::MySqlRow, Column, Row, TypeInfo};
use std::{collections::HashMap, env, error::Error, fs, process};

#[derive(Debug)]
struct Tablecol {
    name: String,
    display_width: i8,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let db_url = get_database_url();

    // Create a connection pool
    clear_term();
    println!("Connecting...");
    let pool = sqlx::mysql::MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;
    clear_term();

    loop {
        let query: String = match get_term_input() {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Error: {}", e);
                "".to_string()
            }
        };

        match sqlx::query(&query).fetch_all(&pool).await {
            Ok(v) => handle_query_ok(v),
            Err(e) => {
                dbg!(e);
            } //handle_query_err(e),
        }
    }
}

fn get_database_url() -> String {
    fn get_env_var() -> String {
        match fs::read_to_string(".env") {
            Ok(v) => {
                let v = v.replace(r#"""#, "");
                let pairs: Vec<Vec<&str>> = v
                    .split("\n")
                    .map(|pair| pair.split("=").collect::<Vec<&str>>())
                    .collect();

                let url = match pairs.iter().find(|k| k[0] == "URL") {
                    Some(v) => v[1].to_owned(),
                    _ => "".to_owned(),
                };

                dbg!(&url);

                url
            }
            _ => {
                println!("Please assign a URL environment variable.");
                println!("Template URL: 'mysql://USERNAME:PASSWORD@HOST/DATABASE'");
                process::exit(1);
            }
        }
    }

    match env::var("URL") {
        Ok(v) => String::from(v),
        _ => get_env_var(),
    }
}

fn handle_query_ok(row_vec: Vec<MySqlRow>) -> () {
    if row_vec.len() == 0 {
        return;
    }

    let mut header_data: HashMap<usize, Tablecol> = row_vec
        .first()
        .unwrap()
        .columns()
        .iter()
        .map(|col| {
            let s = col.name().to_owned();
            (
                col.ordinal(),
                Tablecol {
                    name: String::from(&s), // s.clone(),
                    display_width: s.chars().count() as i8,
                },
            )
        })
        .collect::<HashMap<usize, Tablecol>>();

    let body_data: Vec<Vec<String>> = row_vec
        .iter()
        .map(|row| {
            let mut row_data: Vec<String> = Vec::new();

            for col_index in 0..row.len() {
                let col_val: String = get_parsed_row_value(col_index, row);

                update_col_width(col_index, col_val.chars().count(), &mut header_data);

                row_data.push(col_val);
            }

            row_data
        })
        .collect();

    draw_table(header_data, body_data);
}

fn get_parsed_row_value(i: usize, row: &MySqlRow) -> String {
    match row.column(i).type_info().to_string().as_str() {
        "CHAR" | "VARCHAR" | "ENUM" => row
            .try_get_unchecked::<String, _>(i)
            .unwrap_or_else(|_| "NULL".to_string()),
        "INT" | "BIGINT" => row
            .try_get_unchecked::<i32, _>(i)
            .unwrap_or_else(|_| -1)
            .to_string(),
        "TEXT" => row
            .try_get_unchecked::<String, _>(i)
            .unwrap_or_else(|_| "NULL".to_string()),
        _ => {
            if row.column(i).type_info().is_null() {
                "NULL".to_owned()
            } else {
                row.column(i).type_info().name().to_owned()
            }
        }
    }
}

fn update_col_width(col_id: usize, w: usize, header_data: &mut HashMap<usize, Tablecol>) -> () {
    if w as i8 > header_data.get(&col_id).unwrap().display_width {
        header_data.entry(col_id).and_modify(|e| {
            e.display_width = w as i8;
        });
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

fn draw_table(header: HashMap<usize, Tablecol>, body: Vec<Vec<String>>) -> () {
    draw_header(&header);
    for row in body {
        draw_body_row(&row, &header);
        print!("\n");
    }
}

fn draw_header(titles: &HashMap<usize, Tablecol>) -> () {
    print!("|");
    for col_id in 0..titles.len() {
        let data = titles.get(&col_id).unwrap();
        print!(" {} ", data.name);

        let padding: i8 = data.display_width - data.name.chars().count() as i8;
        let mut i: i8 = 0;
        while i < padding {
            print!(" ");
            i = i + 1;
        }

        print!("|");
    }

    print!("\n-");

    let total_table_width: i8 = titles.iter().map(|col| col.1.display_width + 3).sum();
    for _ in 0..total_table_width {
        print!("-");
    }

    print!("\n");
}

fn draw_body_row(row: &Vec<String>, table_data: &HashMap<usize, Tablecol>) -> () {
    print!("|");
    let mut col_id: usize = 0;
    for col_val in row {
        print!(" {} ", col_val);

        let padding: i8 =
            table_data.get(&col_id).unwrap().display_width - col_val.chars().count() as i8;
        let mut i: i8 = 0;
        while i < padding {
            print!(" ");
            i = i + 1;
        }

        print!("|");

        col_id = col_id + 1;
    }
}
