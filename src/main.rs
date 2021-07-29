#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

pub use rocket::http::RawStr;
use rocket::response::Redirect;
pub use url_longener::longen_url;
pub use url_longener::quotes_db;
pub use url_longener::stats;
pub use url_longener::urls_db;


//TODO : make a html/js page and learn how to serve those in rust using rocket
#[get("/")]
fn index() -> String {
    format!("landing")
}

// TODO : Make a favicon with shakespeare
#[get("/favicon.ico")]
fn favicon() -> String {
    format!("Just pretend this is a favicon")
}

#[post("/longen", data = "<original_url>")]
fn longen(original_url: String) -> String {
    let longened_url = match longen_url::longen_url(original_url.as_str()) {
        Ok(url) => url,
        Err(_) => return format!("Please try again later"),
    };
    //TODO : make original_url standard
    if urls_db::insert_new_pair(longened_url.as_str(), original_url.as_str()).is_ok() {
        format!("Accepted {} as {}", longened_url.as_str(), original_url.as_str())
    } else {
        println!("LOG : Could not insert, it already exists");
        format!("This url already exists, we are supposed to give you that URL, but we haven't implemented that yet.")
    }
}

#[get("/<longened_url>")]
fn lookup(longened_url: &RawStr) -> Redirect {
    match urls_db::select_original_url(longened_url.as_str()) {
        Ok(url) => {
            match urls_db::update_clicks(url.clone()){
                Ok(_) => println!("LOG : Clicks updated"),
                Err(e) => println!("LOG : Failed to update click... : {}", e)
            };
            println!("Redirecting to {}", &url);
            Redirect::temporary(format!("http://{}", &url))
        },

        Err(e) => {println!("{}", e); Redirect::temporary("/")},
    }
}

#[get("/<longened_url>/clicks")]
fn clicks(longened_url: &RawStr) -> String {
    match stats::get_clicks_of(longened_url.as_str()) {
        Ok(click_count) => format!("{} has been clicked {} times", longened_url.as_str(), click_count),
        Err(_e) => format!("Could not retrieve the requested data"),
    }
}


#[get("/<longened_url>/full")]
fn full_stats(longened_url: &RawStr) -> String {
    match stats::get_full_stats_of(longened_url.as_str()) {
        Ok(data) => format!(
            "Original URL \t Longened URL \t Clicks\n{}\t{}\t{}\t",
            data.original, data.longened, data.clicks
        ),
        Err(e) => format!("Could not retrieve the requested data {}", e),
    }
}

fn main() {
    match urls_db::init_database() {
        Ok(_) => println!("LOG : urls database started"),
        Err(_) => panic!("urls database failed to start"),
    };
    match quotes_db::init_database() {
        Ok(_) => println!("LOG : quotes database started"),
        Err(e) => panic!("quotes database failed to start {}", e),
    };

    rocket::ignite()
        .mount("/", routes![index, favicon, lookup, longen])
        .mount("/stats", routes![clicks, full_stats])
        .launch();
}
