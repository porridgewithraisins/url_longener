pub use rusqlite::{params, Connection, Result};
pub use std::fs::File;
pub use std::io::{BufRead, BufReader};

pub use crate::constants;

pub type Quote = String;

fn get_database_path() -> String {
    constants::QUOTES_DATABASE_PATH.to_string()
}

fn get_database_connection() -> Result<Connection> {
    let database_path = get_database_path();
    Connection::open(database_path)
}

fn get_quotes_file_path() -> String {
    constants::QUOTES_FILE_PATH.to_string()
}

fn populate_table(conn: Connection) -> Result<()> {
    let insert = |quote: String| -> Result<()> {
        let query = "INSERT INTO quotes (quote) VALUES (?1)";
        conn.execute(query, params![quote])?;
        Ok(())
    };

    let keep_only_alphanumeric = |s : &str| -> String {
        let x : String = s.chars().filter(|ch| ch.is_alphanumeric()||ch == &' ').collect();
        x.replace(" ", "-")
    };

    let f = File::open(get_quotes_file_path()).expect("Unable to open file");
    let mut f = BufReader::new(f);
    let mut buffer = String::new();

    while let Ok(left) = f.read_line(&mut buffer) {
        if left == 0 {
            break;
        }
        insert(keep_only_alphanumeric(&buffer))?;
        buffer.clear();
    }
    Ok(())
}
fn initialize_table(conn: Connection) -> Result<()> {
    conn.execute("CREATE TABLE IF NOT EXISTS quotes(quote TEXT UNIQUE);", [])?;
    Ok(())
}

fn is_table_empty(conn : Connection) -> Result<bool>{
    let query = "SELECT count(*) FROM quotes;";
    let count :i32 = conn.query_row(query, [], |row| row.get(0))?;
    Ok(count == 0)
}

pub fn init_database() -> Result<()> {
    initialize_table(get_database_connection()?)?;
    if is_table_empty(get_database_connection()?)?{
        populate_table(get_database_connection()?)?;
        println!("Populating table, it was seen to be empty.")
    }
    else {
        println!("Table already has rows, not populating it.")
    }
    Ok(())
}

pub fn select_quote() -> Result<Quote> {
    let conn = get_database_connection()?;
    let query = "SELECT quote FROM quotes ORDER BY RANDOM() LIMIT 1";
    conn.query_row(query, [], |row| row.get(0))
}

// pub fn delete_quote() -> Result<()> {
//     let conn = get_database_connection()?;
//     let query = "DELETE FROM quotes WHERE ROWID = 1";
//     conn.execute(query, [])?;
//     Ok(())
// }
