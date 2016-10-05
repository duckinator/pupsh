fn parse_line(string: String) {
    let mut tokens = Vec::new();

    println!("{}", string);
    tokens.push("echo");
    tokens.push("hello, world!");

    tokens;
}

fn execute_line(parts: Vec<String>) {
    let command = &parts[0];
    let args    = &parts[1..-1];

//    print!("{} {}", command, args);
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
