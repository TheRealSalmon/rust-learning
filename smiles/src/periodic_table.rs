use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

#[derive(Debug)]
pub struct PeriodicTable<'a> {
    atomic_symbols: HashMap<usize, &'a str>,
    // atomic_masses: HashMap<usize, &'a str>,
}

impl PeriodicTable<'_> {
    pub fn new() {
        let mut atomic_symbols: HashMap<usize, &'_ str> = HashMap::new();

        let file = File::open("src/ref_files/periodic_table.csv").unwrap();
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line.unwrap();
            println!("{}", line)
        };

        // Self{
            // atomic_symbols: atomic_symbols,
            // atomic_masses: todo!(),
        // }
    }
}