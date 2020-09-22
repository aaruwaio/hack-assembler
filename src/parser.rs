use std::io::prelude::*;
use std::io::BufReader;

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
}
