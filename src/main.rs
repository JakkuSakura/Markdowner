use std::io::{Read, stdin};

mod parser;
fn main() {

    let stdin = std::io::stdin();

    let mut buf = String::new();
    match stdin.lock().read_to_string(&mut buf) {
        Ok(_) => {}
        Err(err) => panic!(err)
    }
    let mut parser = parser::Parser::new(&buf);
    parser.build();
    let mut output = String::new();
    parser.output(&mut output);
    print!("{}", output)
}
