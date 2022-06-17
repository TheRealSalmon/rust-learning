use crate::gen_chem::{PeriodicTable};
use crate::bond::Bond;

#[derive(Debug)]
pub enum Hybridization {
    Unspecified,
    S,
    SP,
    SP2,
    SP3,
    SP3D,
    SP3D2,
    Other,
}

#[derive(Debug)]
pub enum Chirality {
    Unspecified,
    TetCW,
    TetCCW,
    Other,
}

#[derive(Debug)]
pub struct Atom {
    pub number: usize,
    pub symbol: String,
    pub isotope: isize,
    pub mass: f64,
    pub formal_charge: isize,
    pub hybridization: Hybridization,
    pub chirality: Chirality,
    pub aromatic: bool,
    pub ring: usize,
    pub index: isize,
    // pub neighbors: Vec<&'a Atom<'a>>,
    // pub bonds: Vec<&'a Bond<'a>>,
    // pub mol: Mol,
}

impl Atom {
    pub fn from_symbol(symbol: &str) -> Self {
        let number = PeriodicTable::convert_symbol_to_number(symbol);
        let symbol = String::from(symbol);
        let isotope = 0;
        let mass: f64 = PeriodicTable::mass_from_symbol(&symbol);
        let formal_charge = 0;
        let hybridization = Hybridization::Unspecified;
        let chirality = Chirality::Unspecified;
        let aromatic = false;
        let ring = 0;
        let index = -1;
        // let neighbors = vec![];
        // let bonds = vec![];
        Self {
            number,
            symbol,
            isotope,
            mass,
            formal_charge,
            hybridization,
            chirality,
            aromatic,
            ring,
            index,
            // neighbors,
            // bonds,
        }
    }

    pub fn atomic_number(&self) -> usize {
        self.number
    }

    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn isotope(&self) -> isize {
        self.isotope
    }

    pub fn mass(&self) -> f64 {
        self.mass
    }

    pub fn formal_charge(&self) -> isize {
        self.formal_charge
    }

    pub fn hybridization(&self) -> &Hybridization {
        &self.hybridization
    }

    pub fn chirality(&self) -> &Chirality {
        &self.chirality
    }

    pub fn aromatic(&self) -> bool {
        self.aromatic
    }

    pub fn ring(&self) -> usize {
        self.ring
    }

    pub fn index(&self) -> isize {
        self.index
    }

    // pub fn neighbors(&self) -> &Vec<&Atom> {
    //     &self.neighbors
    // }

    // pub fn num_neighbors(&self) -> usize {
    //     self.neighbors.len()
    // }

    // pub fn num_hydrogens(&self) -> usize {
    //     let mut count = 0;
    //     for neighbor in self.neighbors {
    //         if neighbor.symbol == "H" {
    //             count += 1;
    //         }
    //     }
    //     count
    // }

    // pub fn add_neighbor(&mut self, neighbor: &Atom) {
    //     self.neighbors.push(neighbor);
    // }

    // pub fn mol(&self) -> Mol {

    // }
}