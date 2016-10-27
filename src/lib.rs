pub fn split_if_not_in_string(string: String, delimiter: char) -> Vec<String> {
    let mut parts = Vec::new();
    let mut part = String::new();
    let mut in_string = false;
    let last_chr: Option<char> = None;

    for chr in string.chars() {
        if chr == '\\' {
            continue;
        }

        if in_string {
            if chr == delimiter {
                parts.push(part.clone());

                let mut part = String::new();

                continue;
            }

            part.push(chr);

            if chr == '"' || chr == '\'' {
                match last_chr {
                    Some('\\') => { }, // No-op if the quote is escaped
                    _ => {
                        in_string = !in_string;
                    },
                }
            }
        }

        let last_chr = Some(chr);
    }

    parts
}

#[test]
fn test_sifnis() {
    // should return a Vector containing:
    //   "foo \"a | b\" "
    //   " bar \"c | d\" "
    //   " baz \"e | f\""
    let ret = split_if_not_in_string("foo \"a | b\" | bar \"c | d\" | baz \"e | f\"", '|');

    assert_eq!(ret.len(), 3);
    assert_eq!(ret[0],  "foo \"a | b\" ");
    assert_eq!(ret[1], " bar \"c | d\" ");
    assert_eq!(ret[2], " baz \"e | f\"");
}

pub fn parse_line(string: String) -> Vec<String> {
    /*let mut inside_string = false;
    //let mut inside_
    let ref last_chr;
    let mut part = "";

    for chr in string.chars() {
        if inside_string {
            if last_chr == "\\" && chr == "\"" {

            }
            part += chr
        }

        match chr {
            "\"" => {
                part += chr
            },
            x => {
            }
        }

        last_chr = chr;
    }*/

    string.split_whitespace().map(String::from).collect()
}

pub fn execute_line(parts: Vec<String>) {
    if parts.len() == 0 {
        return;
    }

    let ref command = parts[0];
    let ref args    = &parts[1..];

    println!("{} {:?}", command, args);
}

