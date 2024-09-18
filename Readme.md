# protein-renderer


## Examples

```sh
cargo run --example bevy_3d
cargo run --example bevy_protein
cargo run --example bevy_protein_mesh
cargo run --example bevy_protein_mesh_camera

```

![](docs/images/protein_01.png)


## Design

- [moviewspec](https://github.com/molstar/mol-view-spec/)
- [molstar dev docs](https://molstar.org/docs/)
- selection --> Loci
- representation --> geometry + material
-



## MoviewSpec Overview:
  - A structure (needs to be abel to be downloaded and parse)
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

  - Union component expression: a combination of the above.
  - Representation ( enum? ):
    - ball and stick
    - spheres
    - other


    ```
    ─root {}
     ├──download {url: "https://www.ebi.ac.uk/pdbe/entry-files/1cbs.bcif"}
     │  └──parse {format: "bcif"}
     │     └──structure {type: "model"}
     │        ├──component {selector: "polymer"}
     │        │  ├──representation {type: "cartoon"}
     │        │  │  ├──color {color: "green"}
     │        │  │  └──color {selector: {label_asym_id: "A", beg_label_seq_id: 1, end_label_seq_id: 50}, color: "#6688ff"}
     │        │  └──label {text: "Protein"}
     │        └──component {selector: "ligand"}
     │           ├──representation {type: "ball_and_stick"}
     │           │  └──color {color: "#cc3399"}
     │           └──label {text: "Retinoic Acid"}
     ├──canvas {background_color: "#ffffee"}
     └──camera {target: [17,21,27], position: [41,34,69], up: [-0.129,0.966,-0.224]}
    ```
