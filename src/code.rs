use std::collections::HashMap;

lazy_static! {
    pub static ref DEST_MAP: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("null", "000");
        m.insert("M", "001");
        m.insert("D", "010");
        m.insert("MD", "011");
        m.insert("A", "100");
        m.insert("AM", "101");
        m.insert("AD", "110");
        m.insert("AMD", "111");
        m
    };
    pub static ref COMP_MAP: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("0", "0101010");
        m.insert("1", "0111111");
        m.insert("-1", "0111010");
        m.insert("D", "0001100");
        m.insert("A", "0110000");
        m.insert("!D", "0001101");
        m.insert("!A", "0110001");
        m.insert("-D", "0001111");
        m.insert("-A", "0110011");
        m.insert("D+1", "0011111");
        m.insert("A+1", "0110111");
        m.insert("D-1", "0001110");
        m.insert("A-1", "0110010");
        m.insert("D+A", "0000010");
        m.insert("D-A", "0010011");
        m.insert("A-D", "0000111");
        m.insert("D&A", "0000000");
        m.insert("D|A", "0010101");
        m.insert("M", "1110000");
        m.insert("!M", "1110001");
        m.insert("-M", "1110011");
        m.insert("M+1", "1110111");
        m.insert("M-1", "1110010");
        m.insert("D+M", "1000010");
        m.insert("D-M", "1010011");
        m.insert("M-D", "1000111");
        m.insert("D&M", "1000000");
        m.insert("D|M", "1010101");
        m
    };
    pub static ref JUMP_MAP: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("null", "000");
        m.insert("JGT", "001");
        m.insert("JEQ", "010");
        m.insert("JGE", "011");
        m.insert("JLT", "100");
        m.insert("JNE", "101");
        m.insert("JLE", "110");
        m.insert("JMP", "111");
        m
    };
}

pub fn dest(mnemonic: String) -> String {
    let key: &str = &mnemonic;
    (*(DEST_MAP.get(key).unwrap())).to_string()
}

pub fn comp(mnemonic: String) -> String {
    let key: &str = &mnemonic;
    (*(COMP_MAP.get(key).unwrap())).to_string()
}

pub fn jump(mnemonic: String) -> String {
    let key: &str = &mnemonic;
    (*(JUMP_MAP.get(key).unwrap())).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dest_test() {
        assert_eq!("000", dest("null".to_string()));
        assert_eq!("001", dest("M".to_string()));
    }

    #[test]
    fn comp_test() {
        assert_eq!("0101010", comp("0".to_string()));
        assert_eq!("1110111", comp("M+1".to_string()));
    }

    #[test]
    fn jump_test() {
        assert_eq!("000", jump("null".to_string()));
        assert_eq!("111", jump("JMP".to_string()));
    }
}
