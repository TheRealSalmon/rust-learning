use crate::atom::Atom;

#[derive(Debug)]
pub enum Stereo {
    None,
    Unspecified,
    Wedge,
    Dash,
    Cross,
    E,
    Z,
}

#[derive(Debug)]
pub enum BondType {
    Single,
    Aromatic,
    Double,
    Triple,
}

#[derive(Debug)]
pub struct Bond {
    // pub atom_1: &'a Atom,
    // pub atom_2: &'a Atom,
    pub bond_type: BondType,
    pub stereo: Stereo,
    pub ring: usize,
    pub index: isize,
    // pub mol: Mol,
}

impl Bond {
    pub fn new(
        // atom_1: &'_ Atom,
        // atom_2: &'_ Atom,
        bond_type: BondType,
        stereo: Stereo,
        ring: usize,
        index: isize,
    ) -> Self {
        Self {
            // atom_1,
            // atom_2,
            bond_type,
            stereo,
            ring,
            index
        }
    }

    // pub fn atom_1(&self) -> &Atom {
    //     self.atom_1
    // }

    // pub fn atom_2(&self) -> &Atom {
    //     self.atom_2
    // }

    // pub fn other_atom(&self, atom: &Atom) -> &Atom {

    // }

    // pub fn atom_1_index(&self) -> isize {
    //     self.atom_1.index()
    // }

    // pub fn atom_2_index(&self) -> isize {
    //     self.atom_2.index()
    // }

    pub fn bond_type(&self) -> &BondType {
        &self.bond_type
    }

    pub fn stereo(&self) -> &Stereo {
        &self.stereo
    }

    pub fn ring(&self) -> usize {
        self.ring
    }

    pub fn index(&self) -> isize {
        self.index
    }

    pub fn aromatic(&self) -> bool {
        matches!(self.bond_type(), BondType::Aromatic)
    }

    // pub fn mol(&self) -> Mol {

    // }
}