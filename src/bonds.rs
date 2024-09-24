use nalgebra::Point3;
use pdbtbx::{Atom, Element, PDB};
use std::collections::HashMap;

struct Bond {
    atom1_index: usize,
    atom2_index: usize,
}

fn detect_bonds(pdb: &PDB) -> Vec<Bond> {
    let mut bonds = Vec::new();
    let max_lengths = create_max_bond_length_map();
    let atoms: Vec<&Atom> = pdb.atoms().collect();

    for (i, atom1) in atoms.iter().enumerate() {
        for (j, atom2) in atoms.iter().enumerate().skip(i + 1) {
            if is_bond(atom1, atom2, &max_lengths) {
                bonds.push(Bond {
                    atom1_index: i,
                    atom2_index: j,
                });
            }
        }
    }

    bonds
}

fn is_bond(atom1: &Atom, atom2: &Atom, max_lengths: &HashMap<(Element, Element), f32>) -> bool {
    let distance = atom1.distance(atom2);

    // Check against maximum bond length
    let max_length = max_lengths
        .get(&(atom1.element(), atom2.element()))
        .or_else(|| max_lengths.get(&(atom2.element(), atom1.element())))
        .unwrap_or(&1.9); // Default max length

    if distance > *max_length {
        return false;
    }

    // Special case for metals (simplified)
    if is_metal(&atom1.element()) || is_metal(&atom2.element()) {
        return distance < 3.5;
    }

    // Reject bonds between atoms from non-adjacent residues (except for disulfide bonds)
    if (atom1.residue_number().unwrap_or(0) as i32 - atom2.residue_number().unwrap_or(0) as i32)
        .abs()
        > 1
    {
        if !(atom1.element() == Element::S && atom2.element() == Element::S && distance < 2.5) {
            return false;
        }
    }

    true
}

fn create_max_bond_length_map() -> HashMap<(Element, Element), f32> {
    let mut map = HashMap::new();
    map.insert((Element::C, Element::C), 1.9);
    map.insert((Element::C, Element::N), 1.7);
    map.insert((Element::C, Element::O), 1.6);
    map.insert((Element::C, Element::S), 1.8);
    map.insert((Element::N, Element::N), 1.6);
    map.insert((Element::N, Element::O), 1.5);
    map.insert((Element::O, Element::O), 1.5);
    map.insert((Element::S, Element::S), 2.5); // For disulfide bonds
                                               // Add more pairs as needed
    map
}

fn is_metal(element: &Element) -> bool {
    matches!(
        element,
        Element::Na
            | Element::Mg
            | Element::K
            | Element::Ca
            | Element::Mn
            | Element::Fe
            | Element::Co
            | Element::Ni
            | Element::Cu
            | Element::Zn
    )
}

// Example usage
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pdb = pdbtbx::open("path/to/your/pdb/file.pdb")?;
    let bonds = detect_bonds(&pdb);
    println!("Detected {} bonds", bonds.len());

    // Print some bond details
    for (index, bond) in bonds.iter().enumerate().take(5) {
        let atom1 = pdb.atoms().nth(bond.atom1_index).unwrap();
        let atom2 = pdb.atoms().nth(bond.atom2_index).unwrap();
        println!(
            "Bond {}: {} - {} (Distance: {:.2}Å)",
            index,
            atom1.name(),
            atom2.name(),
            atom1.distance(atom2)
        );
    }

    Ok(())
}
