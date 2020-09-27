use std::collections::HashMap;

pub struct SymbolTable {
    pub symbol_addresses: HashMap<String, String>,
}

impl SymbolTable {
    pub fn new() -> Self {
        let mut m = HashMap::new();
        m.insert("SP".to_string(), "0000000000000000".to_string());
        m.insert("LCL".to_string(), "0000000000000001".to_string());
        m.insert("ARG".to_string(), "0000000000000010".to_string());
        m.insert("THIS".to_string(), "0000000000000011".to_string());
        m.insert("THAT".to_string(), "0000000000000100".to_string());

        m.insert("R0".to_string(), "0000000000000000".to_string());
        m.insert("R1".to_string(), "0000000000000001".to_string());
        m.insert("R2".to_string(), "0000000000000010".to_string());
        m.insert("R3".to_string(), "0000000000000011".to_string());
        m.insert("R4".to_string(), "0000000000000100".to_string());
        m.insert("R5".to_string(), "0000000000000101".to_string());
        m.insert("R6".to_string(), "0000000000000110".to_string());
        m.insert("R7".to_string(), "0000000000000111".to_string());
        m.insert("R8".to_string(), "0000000000001000".to_string());
        m.insert("R9".to_string(), "0000000000001001".to_string());
        m.insert("R10".to_string(), "0000000000001010".to_string());
        m.insert("R11".to_string(), "0000000000001011".to_string());
        m.insert("R12".to_string(), "0000000000001100".to_string());
        m.insert("R13".to_string(), "0000000000001101".to_string());
        m.insert("R14".to_string(), "0000000000001110".to_string());
        m.insert("R15".to_string(), "0000000000001111".to_string());

        m.insert("SCREEN".to_string(), "0100000000000000".to_string());
        m.insert("KBD".to_string(), "0110000000000000".to_string());

        SymbolTable {
            symbol_addresses: m,
        }
    }

    pub fn add_entry(&mut self, symbol: String, address: String) {
        self.symbol_addresses.insert(symbol, address);
    }

    pub fn contains(&self, symbol: String) -> bool {
        self.symbol_addresses.contains_key(&symbol)
    }

    pub fn get_address(&self, symbol: String) -> String {
        match self.symbol_addresses.get(&symbol) {
            Some(address) => address.to_string(),
            None => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn symbol_table_test() {
        let mut symbol_table = SymbolTable::new();
        symbol_table.add_entry("test".to_string(), "0000000000010000".to_string());
        assert_eq!(true, symbol_table.contains("test".to_string()));
        assert_eq!(
            "0000000000010000".to_string(),
            symbol_table.get_address("test".to_string())
        );
    }
}
