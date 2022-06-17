use buli::gen_chem::PeriodicTable;

#[test]
fn test_convert_number_to_symbol() {
    let h_number = 1;
    let h_symbol = "H";
    assert_eq!(PeriodicTable::convert_number_to_symbol(&h_number), h_symbol);
}

#[test]
fn test_convert_symbol_to_number() {
    let h_symbol = "H";
    let h_number = 1;
    assert_eq!(PeriodicTable::convert_symbol_to_number(&h_symbol), h_number);
}

#[test]
fn test_mass_from_number() {
    let h_number = 1;
    let h_mass = 1.007;
    assert_eq!(PeriodicTable::mass_from_number(&h_number), h_mass);
}

#[test]
fn test_mass_from_symbol() {
    let h_symbol = "H";
    let h_mass = 1.007;
    assert_eq!(PeriodicTable::mass_from_symbol(&h_symbol), h_mass);
}
