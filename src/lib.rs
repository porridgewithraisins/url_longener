// cleans up the URL to standard form (http://example.org) before stowing
pub mod clean_url;
// holds constants such as paths to the SQLite file, and so on
pub mod constants;
// returns a random shakespeare quote
pub mod longen_url;
// handles SQLite API for the quotes store
pub mod quotes_db;
// exports methods that give statistics about the clicks a URL has got through
// the longened url
pub mod stats;
//Handles SQLite API for the URL store
pub mod urls_db;
