
pub fn get_longened_url(short_url : &str) -> String{
    let mut result = String::new();
    for _ in 0..3{
        result.push_str(short_url);
    }
    result 
}