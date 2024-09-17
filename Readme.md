# protein-renderer


## Examples

```sh
cargo run --example bevy_3d
cargo run --example bevy_protein
cargo run --example bevy_protein_mesh

```


## Design

- [moviewspec](https://github.com/molstar/mol-view-spec/)
- [molstar dev docs](https://molstar.org/docs/)
- selection --> Loci
- representation --> geometry + material
-



## MoviewSpec voerview:
  - Static selector: "all", "polymer", "protein", "nucleic", "branched", "ligand", "ion", "water"
  - Component expression:
  ```
  {
      label_entity_id?: str,    // Entity identifier
      label_asym_id?: str,      // Chain identifier in label_* numbering
      auth_asym_id?: str,       // Chain identifier in auth_* numbering
      label_seq_id?: int,       // Residue number in label_* numbering
      auth_seq_id?: int,        // Residue number in auth_* numbering
      pdbx_PDB_ins_code?: str,  // PDB insertion code
      beg_label_seq_id?: int,   // Minimum label_seq_id (inclusive), leave blank to start from the beginning of the chain
      end_label_seq_id?: int,   // Maximum label_seq_id (inclusive), leave blank to go to the end of the chain
      beg_auth_seq_id?: int,    // Minimum auth_seq_id (inclusive), leave blank to start from the beginning of the chain
      end_auth_seq_id?: int,    // Maximum auth_seq_id (inclusive), leave blank to go to the end of the chain
      label_atom_id?: str,      // Atom name like 'CA', 'N', 'O', in label_* numbering
      auth_atom_id?: str,       // Atom name like 'CA', 'N', 'O', in auth_* numbering
      type_symbol?: str,        // Element symbol like 'H', 'HE', 'LI', 'BE'
      atom_id?: int,            // Unique atom identifier (_atom_site.id)
      atom_index?: int,         // 0-based index of the atom in the source data
  }
  ```
