use std::fmt;

use crate::mol::Mol;

#[derive(Debug)]
pub struct SmilesParseError;

impl fmt::Display for SmilesParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error in parsing SMILES string")
    }
}