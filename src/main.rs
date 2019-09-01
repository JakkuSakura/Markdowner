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

impl Buf for Vec<u8> {
    fn push_vec(&mut self, s: &InnerBuffer) {
        for e in s {
            self.push(*e);
        }
    }
    fn push_str(&mut self, s: &str) {
        for e in s.as_bytes() {
            self.push(*e);
        }
    }

    fn push_char(&mut self, ch: char) {
        self.push(ch as InnerByte)
    }

    fn push(&mut self, ch: u8) {
        self.push(ch);
    }
}

fn main() {
    let input = read_from_file("temp/input.txt");
//    println!("Input: {}", input);
    let parser = parser::Parser { raw_text: input.into_bytes() };
    let mut buf: Vec<u8> = vec![];
    parser.parse(&mut buf);
    let mut output = File::create("temp/output.html").unwrap();
    output.write(buf.as_slice()).unwrap();

}
