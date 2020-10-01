use std::collections::HashMap;

pub struct SymbolTable {
    pub symbol_addresses: HashMap<String, i32>,
}

impl SymbolTable {
    pub fn new() -> Self {
        let mut m = HashMap::new();
        m.insert("SP".to_string(), 0);
        m.insert("LCL".to_string(), 1);
        m.insert("ARG".to_string(), 2);
        m.insert("THIS".to_string(), 3);
        m.insert("THAT".to_string(), 4);

        m.insert("R0".to_string(), 0);
        m.insert("R1".to_string(), 1);
        m.insert("R2".to_string(), 2);
        m.insert("R3".to_string(), 3);
        m.insert("R4".to_string(), 4);
        m.insert("R5".to_string(), 5);
        m.insert("R6".to_string(), 6);
        m.insert("R7".to_string(), 7);
        m.insert("R8".to_string(), 8);
        m.insert("R9".to_string(), 9);
        m.insert("R10".to_string(), 10);
        m.insert("R11".to_string(), 11);
        m.insert("R12".to_string(), 12);
        m.insert("R13".to_string(), 13);
        m.insert("R14".to_string(), 14);
        m.insert("R15".to_string(), 15);

        m.insert("SCREEN".to_string(), 16384);
        m.insert("KBD".to_string(), 24576);

        SymbolTable {
            symbol_addresses: m,
        }
    }

    pub fn add_entry(&mut self, symbol: String, address: i32) {
        self.symbol_addresses.insert(symbol, address);
    }

    pub fn contains(&self, symbol: String) -> bool {
        self.symbol_addresses.contains_key(&symbol)
    }

    pub fn get_address(&self, symbol: String) -> i32 {
        match self.symbol_addresses.get(&symbol) {
            Some(address) => *address,
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
        symbol_table.add_entry("test".to_string(), 16);
        assert_eq!(true, symbol_table.contains("test".to_string()));
        assert_eq!(16, symbol_table.get_address("test".to_string()));
    }
}
