extern crate url;

pub fn clean_url<S:Into<String>>(_original_url : S) -> String {
    todo!()
}
/*
TODO : Before storing away the original URL, must make it of the 
standard form, so that redirection works. For now, it relies on the 
original POST request containing the scheme. I haven't been able to
figure out how to make Rocket::response::redirect take care of this.
So have to do it myself
*/