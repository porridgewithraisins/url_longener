use rusqlite::{params, Connection, Result};
use std::env;

#[derive(Debug, PartialEq, Eq)]
pub struct StoredData {
    pub original_url : String,
    pub longened_url : String
}

impl StoredData {

    pub fn new(original_url: String, longened_url: String) -> Self {
        Self { original_url, longened_url }
    }

    pub fn empty_select_result() -> Self {
        let failure_placeholder = "NULL";
        Self {
            original_url: (String::from(failure_placeholder)),
            longened_url: (String::from(failure_placeholder)),
        }
    }
    
}

fn get_database_path()-> String {
    env::var("DATABASE_PATH")
    .unwrap_or_else(|e| {
        panic!("could not find env var DATABASE_PATH : {}", e)
    })
}

fn get_db_context() -> Connection{
    match Connection::open(get_database_path()){
        Ok(conn) => conn,
        Err(_e) => panic!("Cannot open database connection")
    }
}



pub fn init() -> Result<()> {
    /*Initialize database connection at ./urls.sqlite */
   
    let conn = get_db_context();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS urls(
            original_url TEXT PRIMARY KEY,
            longened_url TEXT UNIQUE,
            clicks INTEGER DEFAULT 0);"
        , []
    )?;

    Ok(())
}

fn insert(original_url : String, longened_url : String) -> Result<()> {
    /*Private function to insert into table, avoids type mismatches with changing API*/

    let conn = get_db_context();
    let insert_query = "INSERT INTO urls (original_url, longened_url) VALUES (?1, ?2)";
    conn.execute(insert_query, params![&original_url, &longened_url])?;
    
    Ok(())

}

pub fn put_urls(original_url : &str, longened_url : &str){
    /*Exposed function for inserting into database, alwats takes string slices*/
    let _ = insert(original_url.to_string(), longened_url.to_string());
    println!("inserted {}, {}", original_url.clone(), longened_url.clone());

}



fn update(longened_url : String) -> Result<()>{
    let conn = get_db_context();

    let update_query = "UPDATE urls SET clicks = clicks + 1 
                            WHERE longened_url = (?1)";

    conn.execute(update_query, params![&longened_url])?;

    Ok(())
}

pub fn update_clicks(longened_url : &str){
    let _ = update(longened_url.to_string());    
}


fn select_urls(longened_url : String) -> Result<StoredData> {
    /*Private function for looking up original url of a longened one*/

    let conn = get_db_context();

    let select_query = "SELECT original_url, longened_url from urls 
    WHERE longened_url = (?1)";

    let mut statement = conn.prepare(select_query)?;

    let mut rows = statement.query(params![&longened_url])
                            .and_then(|row| Ok(StoredData::new(row.get(0)?, row.get(1)?)))?;

    if let Some(row) = rows.next()?{
        Ok(StoredData::new(row.get(0)?, row.get(1)?))
    }
    else {
        Ok(StoredData::empty_select_result())
    }

    }

pub fn get_urls(longened_url : &str) -> StoredData {
    /* Exposed function to select from database, always takes a string slice
    returns : (status code, payload) */

    let payload = select_urls(longened_url.to_string());

    match payload {
        Ok(payload) => payload,
        Err(_) => StoredData::empty_select_result()
    }
}

fn select_clicks(longened_url : String) -> Result<i32> {
    let conn = get_db_context();
    todo!()
}

pub fn get_clicks(longened_url : &str) -> i32 {
    todo!()
}

fn select_last() -> Result<String>{
    let conn = get_db_context();
    let query = "SELECT * FROM table ORDER BY column DESC LIMIT 1;";
    let mut statement = conn.prepare(sql)?;
    

    conn.execute(query,[])?;

}

pub fn get_last() -> String{
    todo!()
}