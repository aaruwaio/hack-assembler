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

    let mut parser = parser::Parser::new(input_file);

    loop {
        parser.advance();
        if !parser.has_more_commands {
            break;
        }

        match parser.command_type() {
            parser::CommandType::Acommand => {
                output_file.write(parser.symbol().as_bytes()).unwrap();
            }
            parser::CommandType::Ccommand => {
                output_file
                    .write(code::dest(parser.dest()).as_bytes())
                    .unwrap();
            }
        };

        // output_file.write(binary_code).unwrap();
    }
}
