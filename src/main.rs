use hack_assembler::{code, parser};
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file_name = &args[1];
    let input_file = BufReader::new(fs::File::open(input_file_name).unwrap());

    let output_file_name = input_file_name.replace("asm", "hack");
    let mut output_file = BufWriter::new(fs::File::create(output_file_name).unwrap());

    let b = "test".as_bytes();
    output_file.write(b).unwrap();

    let parser = parser::Parser::new(input_file);
}
