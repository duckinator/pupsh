use std::io::{self, Write};

fn parse_line(string: String) -> Vec<String> {
    // TODO: Implement a proper parser. Maybe us nom?
    string.split_whitespace().map(String::from).collect()
}

fn execute_line(parts: Vec<String>) {
    let ref command = parts[0];
    let ref args    = &parts[1..];

    println!("{} {:?}", command, args);
}

fn main() {
    loop {
        let mut input = String::new();

        print!("$ ");

        io::stdout().flush().unwrap();

        match io::stdin().read_line(&mut input) {
            Ok(n) => execute_line(parse_line(input)),
            Err(error) => println!("error: {}", error),
        }

        io::stdout().flush().unwrap();
    }
}
