fn parse_line(string: String) -> Vec<String> {
    let mut tokens = Vec::new();

    println!("{}", string);
    tokens.push(String::from("echo"));
    tokens.push(String::from("hello, world!"));

    tokens
}

fn execute_line(parts: Vec<String>) {
    let ref command = parts[0];
    let ref args    = &parts[1..];

    print!("{} {:?}", command, args);
}

fn main() {
    loop {
        let mut input = String::new();

        match std::io::stdin().read_line(&mut input) {
            Ok(n) => execute_line(parse_line(input)),
            Err(error) => println!("error: {}", error),
        }
    }
}
