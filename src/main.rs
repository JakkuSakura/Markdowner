use std::io::{Read, Write};
use std::fs::File;

mod parser;
use crate::parser::Buf;
use crate::parser::InnerBuffer;
use crate::parser::InnerByte;

fn read_from_file(filename: &str) -> String {
    let mut file = File::open(filename).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    return buf
}

include!("vec_u8_buf.in");

fn main() {
    let input = read_from_file("temp/input.txt");
//    println!("Input: {}", input);
    let mut buf: Vec<u8> = vec![];
    parser::parse(&input.into_bytes(), &mut buf);
    let mut output = File::create("temp/output.html").unwrap();
    output.write(buf.as_slice()).unwrap();

}