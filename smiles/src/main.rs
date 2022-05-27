mod periodic_table;
use periodic_table::PeriodicTable;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
We will create an Atom that can imitate the key RDKit functionalities:
* GetAtomicNum() will return the atomic number
* GetSymbol() will return the atomic symbol
* GetIndex() will return the index of the atom in its Mol
* GetChirality() will return the chirality
* GetHybridization() will return the hybridization
* GetFormalCharge() will return the formal charge
* GetDegree() will return the number of neighbors including Hs
* GetValence() will return the number of bonding interactions
* GetNumHs() will return the total number of Hs
* IsAromatic() will return if the atom is aromatic or not
* GetIsotope() will return the isotope, relative to the major isotope
* GetParentMol() will return the owning Mol
* IsRingAtom() will return if the atom is in a ring
* Need to implement some kind of comparison, allowing loose and strict comparing
* Setters?

We will create a Mol that can imitate the key RDKit functionalities:
* GetAtom() will return an atom given an index
* GetAtoms() will return a list of Atom objects
* GetBond() will return a bond given two indices
* GetBonds() will return a list of Bond objects
* GetNeighbors() will return a list of neighbor atom indices given an index
* GetNumHeavyAtoms() will return the number of heavy atoms
* GetNumAllAtoms() will return the number of atoms including Hs
* GetNumBonds() will return the number of bonds
 */

 #[derive(Debug)]
enum Chirality {
    Unspecified,
    TetrahedralCW,
    TetrahedralCCW,
    Other
}

#[derive(Debug)]
enum Hybridization {
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
pub struct Atom<'a> {
    atomic_num: usize,
    symbol: &'a str,
    index: usize,
    formal_charge: usize,
    degree: usize,
    valence: usize,
    aromatic: bool,
    in_ring: bool,
    chirality: Chirality,
    hybridization: Hybridization,
    isotope: usize,
}

impl Atom<'_> {
    fn new(atomic_num: usize) -> Self {
        Self{
            atomic_num: atomic_num,
            symbol: "",
            index: 0,
            formal_charge: 0,
            degree: 0,
            valence: 0,
            aromatic: false,
            in_ring: false,
            chirality: Chirality::Unspecified,
            hybridization: Hybridization::Unspecified,
            isotope: 0,
        }
    } 
}

#[derive(Debug)]
pub struct Mol {
    adjacency_matrix: Vec<Vec<usize>>,
    atoms: Vec<usize>,
    bonds: Vec<usize>,
}

fn main() {
    // let mut mol = Mol {
    //     adjacency_matrix: vec![
    //         vec![0, 1],
    //         vec![1, 0]
    //     ],
    //     atoms: vec![0, 1],
    //     bonds: vec![2, 3]
    // };
    // mol.adjacency_matrix[1][1] = 1;
    // mol.atoms[0] = 2;
    // mol.bonds[1] = 4;
    // dbg!(mol);
    // let atom: Atom = Atom::new(1);
    // dbg!(atom);
    // let periodic_table: PeriodicTable = PeriodicTable::new();
    let filename: &str = "src/ref_files/periodic_table.csv";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for line in reader.lines() {
        let line = line.unwrap(); // Ignore errors.
        // Show the line and its number.
        // for word in line.split(",") {
        //     dbg!(word);
        // };
        // dbg!(line.split(","));
        dbg!(line.split(",").nth(0));
        // match line.split(",").nth(0) {
        //     Some(word) => println!("{}", word),
        //     None => (),
        // };
        // println!("{:?}", line.split(",").nth(1));
    }
}
