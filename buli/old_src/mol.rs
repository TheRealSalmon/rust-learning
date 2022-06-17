use crate::{atom::Atom, bond::Bond};

#[derive(Debug, Default)]
pub struct Mol {
    atoms: Vec<Atom>,
    bonds: Vec<Bond>,
}

impl Mol {
    pub fn new(atoms: Vec<Atom>, bonds: Vec<Bond>) -> Self {
        Self { atoms, bonds }
    }

    pub fn from_smiles(smiles: &str) -> Self {
        let atoms = vec![];
        let bonds = vec![];



        Self { atoms, bonds }
    }

    pub fn atoms(&self) -> &Vec<Atom> {
        &self.atoms
    }

    pub fn bonds(&self) -> &Vec<Bond> {
        &self.bonds
    }
}