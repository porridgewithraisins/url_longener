use rusqlite::{params, Connection, Result};

#[derive(Debug)]
pub struct StoredData {
    pub original_url : String,
    pub longened_url : String
}

impl StoredData {

    pub fn new(original_url: String, longened_url: String) -> Self {
        Self { original_url, longened_url }
    }

    pub fn select_failure() -> Self {
        let failure_placeholder = "NULL";
        Self {
            original_url: (String::from(failure_placeholder)),
            longened_url: (String::from(failure_placeholder)),
        }
    }
    
}



pub fn init() -> Result<()> {
    /*Initialize database connection at ./urls.sqlite */

    let conn = Connection::open("urls.sqlite")?;
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

    let payload =  get(&longened_url);


    let conn = Connection::open("urls.sqlite")?;
    conn.execute(
        "INSERT INTO urls (original_url, longened_url) VALUES (?1, ?2)"
        , params![&original_url, &longened_url]
    )?;
    
    Ok(())

}

pub fn put_urls(original_url : &str, longened_url : &str) -> Result<()> {
    /*Exposed function for inserting into database, alwats takes string slices*/
    let _ = insert(original_url.to_string(), longened_url.to_string());
    println!("inserted {}, {}", original_url, longened_url);
    Ok(())
}

fn update_clicks(longened_url : &str){
    
}

fn select(longened_url : String) -> Result<StoredData> {
    /*Private function for looking up original url of a longened one*/

    let conn = Connection::open("urls.sqlite")?;
    let mut statement = conn.prepare(
        "SELECT original_url, longened_url from urls 
        WHERE longened_url = (?1)"
    )?;

    let mut rows = statement.query(params![&longened_url])?;


    let mut result = StoredData::select_failure();

    if let Some(row) = rows.next()?{
        result = StoredData::new(row.get(0)?, row.get(1)?);
    }

    Ok(result)
    }



pub fn get(longened_url : &str) -> StoredData {
    /* Exposed function to select from database, always takes a string slice
    returns : (status code, payload) */

    let payload = select(longened_url.to_string());

    match payload {
        Ok(payload) => payload,
        Err(_) => StoredData::select_failure()
    }
}
