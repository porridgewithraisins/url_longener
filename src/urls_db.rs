
pub use rusqlite::{params, Connection};
pub use crate::constants;

pub type URL = String;
pub type Clicks = i32;
pub struct DataModel {
    pub longened: String,
    pub original: String,
    pub clicks: i32,
}

pub fn init_database() -> rusqlite::Result<()> {
    initialize_table(get_database_connection()?)?;
    Ok(())
}

pub fn insert_new_pair<S:Into<String>>(longened_url: S, original_url: S) -> rusqlite::Result<()> {
    let conn = get_database_connection()?;
    let insert_query = "INSERT INTO urls (longened_url, original_url) VALUES (?1, ?2)";
    conn.execute(insert_query, params![longened_url.into(), original_url.into()])?;
    Ok(())
}

pub fn update_clicks<S:Into<String>>(longened_url: S) -> rusqlite::Result<()> {
    let conn = get_database_connection()?;
    let update_clicks_query = "UPDATE urls SET clicks = clicks + 1
        WHERE longened_url = (?1)";
    conn.execute(update_clicks_query, [longened_url.into()])?;
    Ok(())
}

pub fn select_original_url<S:Into<String>>(longened_url: S) -> rusqlite::Result<URL> {
    let conn = get_database_connection()?;
    let query = "SELECT original_url from urls WHERE longened_url = (?1)";
    conn.query_row(query, params![longened_url.into()], |row| row.get(0))
}

pub fn select_longened_url<S:Into<String>>(original_url: S) -> rusqlite::Result<URL> {
    let conn = get_database_connection()?;
    let query = "SELECT longened_url from urls WHERE original_url = (?1)";
    conn.query_row(query, params![original_url.into()], |row| row.get(0))
}

pub fn select_clicks<S:Into<String>>(longened_url: S) -> rusqlite::Result<Clicks> {
    let conn = get_database_connection()?;
    let query = "SELECT clicks from urls WHERE longened_url = (?1)";
    conn.query_row(query, params![longened_url.into()], |row| row.get(0))
}

pub fn select_full_row<S:Into<String>>(longened_url: S) -> rusqlite::Result<DataModel> {
    let conn = get_database_connection()?;
    let query = "SELECT longened_url, original_url, clicks from urls
                    WHERE longened_url = (?1)";
    conn.query_row(query, params![longened_url.into()], |row| {
        Ok(DataModel {
            longened: row.get(0)?,
            original: row.get(1)?,
            clicks: row.get(2)?,
        })
    })
}

// pub fn select_last_insert() -> rusqlite::Result<DataModel> {
//     let conn = get_database_connection()?;
//     let query = "SELECT * FROM table ORDER BY column DESC LIMIT 1;";
//     conn.query_row(query, [], |row| {
//         Ok(DataModel {
//             longened: row.get(0)?,
//             original: row.get(1)?,
//             clicks: row.get(2)?,
//         })
//     })
// }

fn get_database_path() -> String {
    constants::URLS_DATABASE_PATH.to_string()
}

fn get_database_connection() -> rusqlite::Result<Connection> {
    let database_path = get_database_path();
    Connection::open(database_path)
}

fn initialize_table(conn: Connection) -> rusqlite::Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS urls(
            longened_url TEXT PRIMARY KEY,
            original_url TEXT UNIQUE,
            clicks INTEGER DEFAULT 0);",
        [],
    )?;

    Ok(())
}

