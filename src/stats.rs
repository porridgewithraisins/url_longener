pub use crate::urls_db;
pub use rusqlite;

pub fn get_clicks_of<S:Into<String>>(longened_url : S) -> rusqlite::Result<i32>{
    let s = longened_url.into();
    urls_db::select_clicks(s)
}

pub fn get_full_stats_of<S:Into<String>>(longened_url : S) -> rusqlite::Result<urls_db::DataModel> {
    urls_db::select_full_row(longened_url.into())
}

pub fn dump_all() -> rusqlite::Result<String> {
    /*TODO : Dumps all the data in the URLs table*/
    todo!()
}