use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;
use super::db_api;

fn get_shakespeare_file_path() -> String{
    env::var("SHAKESPEARE_FILE_PATH")
        .unwrap_or_else(|e| {panic!("could not find env var SHAKESPEARE_FILE_PATH : {}", e)})
}

// pub fn get_shakespeare_quote() -> String{
    
//     let f = File::open(get_shakespeare_file_path()).expect("Unable to open file");
//     let f = BufReader::new(f);

//     let mut index = 0;
//     // let index_to_serve = get_index_to_serve();
//     let mut quote = String::new();
//     for line in f.lines() {
//         quote = line.expect("Unable to read line");
//         index += 1;
//         if index == index_to_serve{
//             return quote;
//         }
//     }
//     quote
// }