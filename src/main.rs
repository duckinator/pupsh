extern crate pupsh;

use std::io::{self, Write};

fn main() {
    loop {
        let mut input = String::new();

        print!("$ ");

        io::stdout().flush().unwrap();

        match io::stdin().read_line(&mut input) {
            Ok(n) => pupsh::execute_line(pupsh::parse_line(input)),
            Err(error) => println!("error: {}", error),
        }

        io::stdout().flush().unwrap();
    }
}
