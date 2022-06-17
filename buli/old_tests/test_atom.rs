use buli::atom::{Atom, Chirality, Hybridization};

#[test]
fn test_atomic_number() {
    let h_atom = Atom::from_symbol("H");
    assert_eq!(h_atom.atomic_number(), 1);
}

#[test]
fn test_symbol() {
    let h_atom = Atom::from_symbol("H");
    assert_eq!(h_atom.symbol(), "H");
}

#[test]
fn test_isotope() {
    let h_atom = Atom::from_symbol("H");
    assert_eq!(h_atom.isotope(),0);
}

#[test]
fn test_mass() {
    let h_atom = Atom::from_symbol("H");
    assert_eq!(h_atom.mass(), 1.007);
}

#[test]
fn test_formal_charge() {
    let h_atom = Atom::from_symbol("H");
    assert_eq!(h_atom.formal_charge(), 0)
}

#[test]
fn test_hybridization() {
    let h_atom = Atom::from_symbol("H");
    assert!(matches!(h_atom.hybridization(), Hybridization::Unspecified));
}

#[test]
fn test_chirality() {
    let h_atom = Atom::from_symbol("H");
    assert!(matches!(h_atom.chirality(), Chirality::Unspecified));
}

#[test]
fn test_aromatic() {
    let h_atom = Atom::from_symbol("H");
    assert_eq!(h_atom.aromatic(), false);
}

#[test]
fn test_ring() {
    let h_atom = Atom::from_symbol("H");
    assert_eq!(h_atom.ring(), 0);
}

#[test]
fn test_index() {
    let h_atom = Atom::from_symbol("H");
    assert_eq!(h_atom.index(), -1);
}

// #[test]
// fn test_neighbors() {
//     let h_atom = Atom::from_symbol("H");
//     assert!(h_atom.neighbors().is_empty());
// }

// #[test]
// fn test_num_neighbors() {
//     let mut h_atom = Atom::from_symbol("H");
//     assert_eq!(h_atom.num_neighbors(), 0);

//     let c_atom = Atom::from_symbol("C");
//     h_atom.add_neighbor(&c_atom);
//     assert_eq!(h_atom.num_neighbors(), 1);
// }

// #[test]
// fn test_num_hydrogens() {
//     let mut c_atom = Atom::from_symbol("C");
//     let h_1 = Atom::from_symbol("H");
//     let h_2 = Atom::from_symbol("H");
//     c_atom.add_neighbor(&h_1);
//     c_atom.add_neighbor(&h_2);
//     assert_eq!(c_atom.num_hydrogens(), 2);
// }