use std::{fs::File, io::{BufReader, Read}};
use simple_logger::SimpleLogger;
use vm::VM;

mod vm;

fn main() {
    SimpleLogger::from_env().init().unwrap();
    let file = File::open("challenge/challenge.bin").unwrap();

    VM::boot()
        .load_program(read_binary(file))
        .run();

    println!("Hello, world!");
}

fn read_binary(file: File) -> Vec<u16> {
    let buf = BufReader::new(file);
    let mut result = Vec::new();

    let mut iter = buf.bytes().into_iter();

    while let Some(low_byte_r) = iter.next() {
        if let Some(high_byte_r) = iter.next() {
            let low_byte = low_byte_r.unwrap();
            let high_byte = high_byte_r.unwrap();

            result.push(((high_byte as u16) << 8) | low_byte as u16)
        }
    }
    result
}
