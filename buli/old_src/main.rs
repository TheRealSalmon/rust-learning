

use std::error::Error;

// use buli::gen_chem::PeriodicTable;
use buli::mol::Mol;
use buli::{atom::Atom, bond::Bond};

fn main() {
    // let mut chars: Vec<String> = vec![];
    // for char in smiles.chars() {
    //     let char = String::from(char);
    //     if ! PeriodicTable::is_valid_symbol(&char) {
    //         println!("not a valid SMILES letter, {char}");
    //     }
    //     chars.push(char);
    // };
    // dbg!(&chars);

    let smi = "CCC";
    let mol = mol_from_smiles(smi);
    dbg!(mol);
    // let mismatched_brackets = "[CCC";
    // let mol = mol_from_smiles(mismatched_brackets);
    // dbg!(mol);
}

fn mol_from_smiles(smiles: &str) -> Mol {
    if smiles.matches('[').count() != smiles.matches(']').count() ||
        smiles.matches('(').count() != smiles.matches(')').count() {
        return Mol::default();
    };

    let mut atoms: Vec<Atom> = vec![];
    let bonds: Vec<Bond> = vec![];

    let chars: Vec<char> = smiles.chars().collect();
    let mut atom_1 = Atom::from_symbol(&chars[0].to_string());
    atoms.push(atom_1);

    for i in 1..chars.len() {
        // let atom_2 = Atom::from_symbol(chars[i]);
        // let bond = Bond::new(atom_1)
    };

    dbg!(&chars);

    Mol::new(atoms, bonds)
}