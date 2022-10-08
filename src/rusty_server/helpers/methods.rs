use std::collections::HashMap;
use std::fs;

pub fn unparse_headers(headers: &HashMap<String, String>) -> String {
    let mut header_string: String = String::new();
    for (key, value) in headers {
        header_string.push_str(format!("{}: {}\r\n", key, value).as_str());
    }
    header_string
}

pub fn get_file_contents(file_path: &str) -> String {
    fs::read_to_string(file_path).unwrap()
}