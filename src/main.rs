#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

pub use rocket::http::RawStr;
pub use url_longener::db_api::*;
pub use url_longener::longen::*;

#[get("/")]
fn index() -> String {
    format!("landing")
}

#[get("/favicon.ico")]
fn favicon() -> String {
    format!("landing")
}


#[get("/<original_url>")]
fn longen(original_url: &RawStr) -> String {
    let database_path = get_database_path();
    let longened_url = get_longened_url(original_url);
    let conn = match get_database_connection(database_path) {
        Ok(conn) => conn,
        Err(_e) => return format!("ERROR")
    };
    if insert_new_pair(conn, longened_url.clone(), original_url.to_string()).is_ok() {
        format!("Inserted pair {}, {}", longened_url, original_url)
    } else {
        format!("Could not insert.")
    }
}

// #[get("/<longened_url>")]

// fn lookup(longened_url: &RawStr) -> String {
//     todo!()
// }



fn main() {
    let database_path = get_database_path();
    println!("Initiating database at {} ...", &database_path);
    let conn = match get_database_connection(database_path.clone()) {
        Ok(conn) => conn,
        Err(_e) => panic!("failed to initialize database!"),
    };
    println!("database connection succeeded");

    match initialize_table(conn) {
        Ok(()) => (),
        Err(_e) => panic!("rusqlite error : failed to initialize table"),
    };
    println!("Table urls created");

    rocket::ignite()
        .mount("/", routes![index, favicon, longen])
        .launch();
}
