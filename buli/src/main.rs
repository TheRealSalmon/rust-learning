// use buli::{Tree, Node};
use gamma::graph::Graph;
use gamma::traversal::{ DepthFirst };
use chemcore::molecule::{ Atom, Element, Molecule, Error, DefaultMolecule };
use chemcore::daylight::{read_smiles, SmilesInputError};

fn main () -> Result<(), SmilesInputError> {
    /*
                            0
                    1               2
                3       4
            5
    */
    // let node_0 = Node::new(0, vec![(0, 1), (0, 2)]);
    // let node_1 = Node::new(1, vec![(1, 3), (1,4)]);
    // let node_2 = Node::new(2, vec![]);
    // let node_3 = Node::new(3, vec![(3, 5)]);
    // let node_4 = Node::new(4, vec![]);
    // let node_5 = Node::new(5, vec![]);
    // let nodes = vec![node_0, node_1, node_2, node_3, node_4, node_5];

    // let tree = Tree::new(nodes);
    // // let nodes = tree.traverse_breadth();
    // let nodes = tree.traverse_depth();
    // dbg!(&nodes);

    let mol = read_smiles("CCOC", None)?;
    let traversal = DepthFirst::new(&mol, 0).expect("traversal error");

    dbg!(mol.degree(1).unwrap());

    Ok(())
}