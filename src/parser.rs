use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
pub struct Parser {
    pub stream: BufReader<std::fs::File>,
    pub current_command: String,
}

impl Parser {
    pub fn new(f: BufReader<std::fs::File>) -> Self {
        Parser {
            stream: f,
            current_command: String::new(),
        }
    }

    pub fn has_more_commands(&self) -> bool {
        //後で実装
        true
    }

    pub fn advance(&mut self) {
        loop {
            let mut buf = String::new();
            self.stream.read_line(&mut buf).unwrap();
            buf = buf.trim().replace(" ", "").to_string();
            if buf.is_empty() {
                continue;
            }
            match buf.find("//") {
                Some(_) => (),
                None => {
                    self.current_command = buf;
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::BufReader;

    #[test]
    fn advance_test() {
        let stream = BufReader::new(File::open("test/add/Add.asm").unwrap());
        let mut p = Parser::new(stream);
        p.advance();
        assert_eq!("@2", p.current_command);
        p.advance();
        assert_eq!("D=A", p.current_command);
    }
}
