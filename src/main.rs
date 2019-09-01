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
    let input = read_from_stdin();
    let mut parser = parser::Parser{ raw_text: input.into_bytes() };
    let vec = parser.parse();
    println!("Result: {}", String::from_utf8(vec).unwrap());

}
