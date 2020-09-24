use regex::Regex;
use std::io::prelude::*;
use std::io::BufReader;

const DEST_REGEX: &str = r"null=+|M=+|D=+|MD=+|A=+|AM=+|AD=+|AMD=+";
const COMP_REGEX: &str = r"=0$|=1$|=-1$|=D$|=A$|=!D$|=!A$|=-D$|=-A$|=D\+1$|=A\+1$|=D-1$|=A-1$|=D\+A$|=D-A$|=A-D$|=D&A$|=D\|A$|=M$|=!M$|=-M$|=M\+1$|=M-1$|=D\+M$|=D-M$|=M-D$|=D&M$|=D\|M$|0;|1;|-1;|D;|A;|!D;|!A;|-D;|-A;|D\+1;|A\+1;|D-1;|A-1;|D\+A;|D-A;|A-D;|D&A;|D\|A;|M;|!M;|-M;|M\+1;|M-1;|D\+M;|D-M;|M-D;|D&M;|D\|M;";
const JUMP_REGEX: &str = r";null|;JGT|;JEQ|;JGE|;JLT|;JNE|;JLE|;JMP";

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

    pub fn dest(&self) -> String {
        let re = Regex::new(DEST_REGEX).unwrap();
        let caps = re.captures(&self.current_command).unwrap();
        caps.get(0)
            .expect("Unexpected character in input")
            .as_str()
            .replace("=", "")
    }

    pub fn comp(&self) -> String {
        let re = Regex::new(COMP_REGEX).unwrap();
        let caps = re.captures(&self.current_command).unwrap();
        caps.get(0)
            .expect("Unexpected character in input")
            .as_str()
            .replace("=", "")
            .replace(";", "")
    }

    pub fn jump(&self) -> String {
        let re = Regex::new(JUMP_REGEX).unwrap();
        let caps = re.captures(&self.current_command).unwrap();
        caps.get(0)
            .expect("Unexpected character in input")
            .as_str()
            .replace(";", "")
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
        p.current_command = "@2".to_string();
        assert_eq!("2", p.symbol());
    }

    #[test]
    fn dest_test() {
        let mut p = create_parser_instance();
        p.current_command = "null=D-A".to_string();
        assert_eq!("null", p.dest());
        p.current_command = "M=M+1".to_string();
        assert_eq!("M", p.dest());
    }

    #[test]
    #[should_panic]
    fn unexpected_input_test() {
        let mut p = create_parser_instance();
        p.current_command = "hoge".to_string();
        p.dest();
        p.comp();
        p.jump();
    }

    #[test]
    fn comp_test() {
        let mut p = create_parser_instance();
        p.current_command = "D=M".to_string();
        assert_eq!("M", p.comp());
        p.current_command = "0;JMP".to_string();
        assert_eq!("0", p.comp());
        p.current_command = "M=M+1".to_string();
        assert_eq!("M+1", p.comp());
        p.current_command = "D=D|A".to_string();
        assert_eq!("D|A", p.comp());
        p.current_command = "D=D&A".to_string();
        assert_eq!("D&A", p.comp());
        p.current_command = "D=-M".to_string();
        assert_eq!("-M", p.comp());
    }

    #[test]
    fn jump_test() {
        let mut p = create_parser_instance();
        p.current_command = "D;JGT".to_string();
        assert_eq!("JGT", p.jump());
        p.current_command = "0;JMP".to_string();
        assert_eq!("JMP", p.jump());
    }
}
