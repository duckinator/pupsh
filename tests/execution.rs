extern crate pupsh;

#[test]
fn zero_length_line() {
    let tokens = Vec::new();

    pupsh::execute_line(tokens);
}
