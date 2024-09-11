
fn split_string(s: String, delimiter: char, field: usize) -> String {
    let parts: Vec<&str> = s.split(delimiter).collect();
    // Handle the Option type and convert to String
    match parts.get(field) {
        Some(&str) => str.to_string(),
        None => String::new(),
    }
}

fn main() {
    let chunk = split_string("hello,world".to_string(), ',', 1);
    println!("Split string: {}", chunk);
}
