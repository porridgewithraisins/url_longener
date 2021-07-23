#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
use rocket::http::RawStr;
use rocket_codegen::route;

mod store;
mod longen;
#[get("/")]
fn index() -> String{
    format!("landing")
}

#[get("/<original_url>")]
fn longen(original_url: &RawStr) -> String {
    let longened_url = longen::get_longened_url(original_url);

    let _ = store::put_urls(original_url.as_str(), "hi");

    String::from("Success")
}

#[get("/<longened_url>")]
fn lookup(longened_url: &RawStr) -> String {
    let payload = store::get(longened_url.as_str());

    format! ("The original URL is {}", payload.original_url)
}

fn main() {
    match store::init(){
        Ok(()) => println!("Database started..."),
        Err(_) => println!("Error initializing database!"),
    }
    rocket::ignite().mount("/", routes![index])
                    .mount("/longen", routes![longen])
                    .mount("/lookup", routes![lookup])
                    .launch();
}