//! Module for loading PDBs into Bevy via the Plugin system
//!
//! Over time this would be a good candidate for factring out
use super::{ColorScheme, Structure};
use bevy::prelude::*;
use pdbtbx::StrictnessLevel;
use std::path::PathBuf;

// adding this for integration with Bevy
pub struct StructurePlugin {
    initial_files: Vec<PathBuf>,
}

impl StructurePlugin {
    pub fn new() -> Self {
        Self {
            initial_files: Vec::new(),
        }
    }
    pub fn with_file<P: Into<PathBuf>>(mut self, path: P) -> Self {
        // println!("Adding file to StructurePlugin: {:?}", path_buf);
        self.initial_files.push(path.into());
        self
    }
    pub fn with_files<I, P>(mut self, paths: I) -> Self
    where
        I: IntoIterator<Item = P>,
        P: Into<PathBuf>,
    {
        self.initial_files.extend(paths.into_iter().map(Into::into));
        self
    }
}

// adding this for integration with Bevy
impl Plugin for StructurePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(StructureFiles(self.initial_files.clone()))
            .add_systems(Startup, load_initial_proteins)
            .add_event::<LoadProteinEvent>();
    }
}

#[derive(Resource)]
struct StructureFiles(Vec<PathBuf>);

#[derive(Event)]
pub struct LoadProteinEvent(pub PathBuf);

fn load_initial_proteins(
    structure_files: Res<StructureFiles>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    println!(
        "Loading initial proteins. Number of files: {}",
        structure_files.0.len()
    );
    for file_path in &structure_files.0 {
        println!("Attempting to load protein from: {:?}", file_path);
        if let Ok((pdb, _errors)) = pdbtbx::open(
            file_path.to_str().unwrap_or_default(),
            StrictnessLevel::Medium,
        ) {
            println!("Successfully loaded PDB from: {:?}", file_path);
            println!("Number of atoms in PDB: {}", pdb.atom_count());
            let structure = Structure::builder()
                .pdb(pdb)
                .color_scheme(ColorScheme::ByAtomType)
                .build();

            // Generate mesh from the structure
            let mesh = structure.to_mesh();
            println!("Number of verices in the mesh: {}", mesh.count_vertices());

            let mesh_handle = meshes.add(mesh);

            // Create a material
            let material = materials.add(StandardMaterial {
                base_color: Color::srgb(0.8, 0.7, 0.6),
                metallic: 0.1,
                perceptual_roughness: 0.5,
                ..default()
            });

            // Spawn the entity with Structure and PbrBundle
            commands.spawn((
                structure,
                PbrBundle {
                    mesh: mesh_handle,
                    material,
                    ..default()
                },
            ));
            println!("Spawned structure entity with mesh and material");
        } else {
            println!("Failed to load protein file: {:?}", file_path);
        }
    }
}
