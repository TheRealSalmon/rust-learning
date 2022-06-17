use std::{collections::HashMap, fs::File, io::BufReader, io::BufRead};

// refactor with phf crate?
#[derive(Debug)]
pub struct PeriodicTable {
    valid_symbols: Vec<String>,
    symbol_to_number: HashMap<String, usize>,
    number_to_symbol: HashMap<usize, String>,
    number_to_mass: HashMap<usize, f64>,
}

impl PeriodicTable {
    pub fn new() -> Self {
        let mut valid_symbols = vec![];
        let mut symbol_to_number = HashMap::new();
        let mut number_to_symbol = HashMap::new();
        let mut number_to_mass = HashMap::new();
        let file_path = "src/ref_files/periodic_table.csv";
        let file = File::open(file_path)
            .expect("file not found");
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line.unwrap();
            let line: Vec<&str> = line.split(",").collect();
            let symbol = String::from(line[0]);
            let number = line[1].parse::<usize>().unwrap();
            let mass = line[2].parse::<f64>().unwrap();
            valid_symbols.push(String::from(&symbol));
            symbol_to_number.insert(String::from(&symbol), number);
            number_to_symbol.insert(number, String::from(&symbol));
            number_to_mass.insert(number, mass);
        }
        Self { valid_symbols, symbol_to_number, number_to_symbol, number_to_mass }
    }

    pub fn is_valid_symbol(symbol: &str) -> bool {
        let periodic_table = PeriodicTable::new();
        for vs in periodic_table.valid_symbols {
            if vs == symbol {
                return true
            }
        };
        false
    }

    pub fn convert_number_to_symbol(number: &usize) -> String {
        let periodic_table = PeriodicTable::new();
        let symbol = String::from(periodic_table.number_to_symbol.get(number).unwrap());
        symbol
    }

    pub fn convert_symbol_to_number(symbol: &str) -> usize {
        let periodic_table = PeriodicTable::new();
        *periodic_table.symbol_to_number.get(symbol).unwrap()
    }

    pub fn mass_from_number(number: &usize) -> f64 {
        let periodic_table = PeriodicTable::new();
        *periodic_table.number_to_mass.get(number).unwrap()
    }

    pub fn mass_from_symbol(symbol: &str) -> f64 {
        let number = PeriodicTable::convert_symbol_to_number(symbol);
        PeriodicTable::mass_from_number(&number)
    }
}