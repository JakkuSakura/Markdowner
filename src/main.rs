use std::io::{Read, stdin};
use std::fs::File;

mod parser;
fn read_from_stdin() -> String {
    let stdin = std::io::stdin();
    let mut buf = String::new();
    stdin.lock().read_to_string(&mut buf).unwrap();
    buf
}
fn read_from_file(filename: &str) -> String {
    let mut file = File::open(filename).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    buf
}
fn main() {

//    let mut parser = parser::Parser::new(&buf);
//    parser.build();
//    let mut output = String::new();
//    parser.output(&mut output);
//    print!("{}", output)
}
