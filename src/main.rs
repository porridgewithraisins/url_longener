#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
pub use rocket::http::RawStr;
pub use url_longener::store;
pub use url_longener::longen;

#[get("/")]
fn index() -> String{
    format!("landing")
}

#[get("/<original_url>")]

fn longen(original_url: &RawStr) -> String {
    let longened_url = longen::get_longened_url(original_url);

    if store::get_urls(&longened_url) == store::StoredData::empty_select_result(){
        store::put_urls(original_url.as_str(), "hi");
    }

    format!("The longened url is {}", longened_url)
}

#[get("/<longened_url>")]

fn lookup(longened_url: &RawStr) -> String {

    let payload = store::get_urls(longened_url.as_str());
    if payload == store::StoredData::empty_select_result() {
        return format!("URL {} does not exist yet", longened_url);
    }
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
