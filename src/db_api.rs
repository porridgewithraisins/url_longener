use rusqlite::{params, Connection, Result};
pub type URL = Option<String>;
pub type Clicks = Option<i32>;
pub struct DataModel {
    longened: String,
    original: String,
    clicks: i32,
}

pub fn get_database_path() -> String {
    match std::env::var("DATABASE_PATH") {
        Ok(path) => path,
        Err(_e) => String::from("./urls.sqlite"),
    }
}

pub fn get_database_connection(database_path: String) -> Result<Connection> {
    Connection::open(database_path)
}

pub fn initialize_table(conn: Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS urls(
            longened_url TEXT PRIMARY KEY,
            original_url TEXT UNIQUE,
            clicks INTEGER DEFAULT 0);",
        [],
    )?;

    Ok(())
}

pub fn insert_new_pair(conn: Connection, longened_url: String, original_url: String) -> Result<()> {

    let insert_query = "INSERT INTO urls (longened_url, original_url) VALUES (?1, ?2)";
    conn.execute(insert_query, params![longened_url, original_url])?;
    Ok(())
}

pub fn update_clicks(conn: Connection, longened_url: String) -> Result<()> {
    let update_clicks_query = "UPDATE urls SET clicks = clicks + 1
        WHERE longened_url = (?1)";
    conn.execute(update_clicks_query, [longened_url])?;
    Ok(())
}

pub fn select_original_url(conn: Connection, longened_url: String) -> Result<URL> {
    let query = "SELECT original_url from urls WHERE longened_url = (?1)";
    conn.query_row(query, params![longened_url], |row| row.get(0))
}

pub fn select_longened_url(conn: Connection, original_url: String) -> Result<URL> {
    let query = "SELECT longened_url from urls WHERE original_url = (?1)";
    conn.query_row(query, params![original_url], |row| row.get(0))
}

pub fn select_clicks(conn: Connection, longened_url: String) -> Result<Clicks> {
    let query = "SELECT clicks from urls WHERE longened_url = (?1)";
    conn.query_row(query, params![longened_url], |row| row.get(0))
}

pub fn select_full_row(conn: Connection, longened_url: String) -> Result<DataModel> {
    let query = "SELECT (longened_url, original_url, clicks) from urls
                    WHERE longened_url = (?1)";
    conn.query_row(query, params![longened_url], |row| {
        Ok(DataModel {
            longened: row.get(0)?,
            original: row.get(1)?,
            clicks: row.get(2)?,
        })
    })
}
