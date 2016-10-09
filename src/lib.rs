use std::io::{self, Write};

pub fn parse_line(string: String) -> Vec<String> {
    // TODO: Implement a proper parser. Maybe use nom?
    string.split_whitespace().map(String::from).collect()
}

pub fn execute_line(parts: Vec<String>) {
    let ref command = parts[0];
    let ref args    = &parts[1..];

    println!("{} {:?}", command, args);
}

