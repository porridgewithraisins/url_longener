pub use crate::quotes_db;

pub fn longen_url<S: Into<String>>(_original_url: S) -> rusqlite::Result<String> {
    let quote = quotes_db::select_quote()?;
    // If the random selection ends up breaking unique constraints,
    // add more logic here, perhaps a custom must-be-unique RNG
    // thats why original_url is a parameter
    Ok(quote)
}
