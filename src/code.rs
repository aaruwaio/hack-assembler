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
}

pub fn dest(mnemonic: String) -> String {
    let key: &str = &mnemonic;
    (*(DEST_MAP.get(key).unwrap())).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dest_test() {
        assert_eq!("000", dest("null".to_string()));
    }
}
