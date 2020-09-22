use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug, PartialEq)]
pub enum CommandType {
    A_COMMAND,
    C_COMMAND,
}

#[derive(Debug)]
pub struct Parser {
    pub stream: BufReader<std::fs::File>,
    pub current_command: String,
    pub has_more_commands: bool,
}

impl Parser {
    pub fn new(f: BufReader<std::fs::File>) -> Self {
        Parser {
            stream: f,
            current_command: String::new(),
            has_more_commands: true,
        }
    }

    pub fn advance(&mut self) {
        loop {
            let mut buf = String::new();
            self.stream.read_line(&mut buf).unwrap();
            //改行文字が含まれない、つまり入力終了
            if buf.is_empty() {
                self.has_more_commands = false;
                break;
            }
            //改行文字、空白の削除
            buf = buf.trim().replace(" ", "").to_string();
            //空白の行はスキップ
            if buf.is_empty() {
                continue;
            }
            //コメント行はスキップ
            match buf.find("//") {
                Some(_) => continue,
                None => {
                    self.current_command = buf;
                    break;
                }
            }
        }
    }

    pub fn command_type(&self) -> CommandType {
        match self.current_command.chars().nth(0).unwrap() {
            '@' => CommandType::A_COMMAND,
            _ => CommandType::C_COMMAND,
        }
    }

    pub fn symbol(&self) -> String {
        self.current_command.clone().replace("@", "")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::BufReader;

    fn create_parser_instance() -> Parser {
        let stream = BufReader::new(File::open("example/add/Add.asm").unwrap());
        Parser::new(stream)
    }

    #[test]
    fn advance_test() {
        let mut p = create_parser_instance();
        p.advance();
        assert_eq!("@2", p.current_command);
        p.advance();
        assert_eq!("D=A", p.current_command);
        p.advance();
        assert_eq!("@3", p.current_command);
        p.advance();
        assert_eq!("D=D+A", p.current_command);
        p.advance();
        assert_eq!("@0", p.current_command);
        p.advance();
        assert_eq!("M=D", p.current_command);
        p.advance();
        assert_eq!(false, p.has_more_commands);
    }

    #[test]
    fn command_type_test() {
        let mut p = create_parser_instance();
        p.advance();
        assert_eq!(CommandType::A_COMMAND, p.command_type());
        p.advance();
        assert_eq!(CommandType::C_COMMAND, p.command_type());
    }

    #[test]
    fn symbol_test() {
        let mut p = create_parser_instance();
        p.advance();
        assert_eq!("2", p.symbol());
    }
}
