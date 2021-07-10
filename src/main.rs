use std::{
    fs::File,
    io::{BufReader, Read},
};
use vm::VM;

mod vm;

fn main() {
    simple_logging::log_to_file("vm.log", log::LevelFilter::Warn).unwrap();

    let file = File::open("challenge/challenge.bin").unwrap();

    VM::boot().load_program(read_binary(file)).run();

    println!("Hello, world!");
}

fn read_binary(file: File) -> Vec<usize> {
    let buf = BufReader::new(file);
    let mut result: Vec<usize> = Vec::new();

    let mut iter = buf.bytes();

    while let Some(low_byte_r) = iter.next() {
        if let Some(high_byte_r) = iter.next() {
            let low_byte = low_byte_r.unwrap();
            let high_byte = high_byte_r.unwrap();

            result.push((((high_byte as u16) << 8) | low_byte as u16).into())
        }
    }
    result
}
