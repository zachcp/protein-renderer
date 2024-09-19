//! Module for loading PDBs into Bevy via the Plugin system
//!
//! Over time this would be a good candidate for factring out
use super::{ColorScheme, RenderOptions, Structure};
use bevy::prelude::*;
use pdbtbx::StrictnessLevel;
use std::path::Path;
use std::path::PathBuf;

#[derive(Clone)]
pub struct StructureSettings {
    pub render_type: RenderOptions,
    pub color_scheme: ColorScheme,
}
impl Default for StructureSettings {
    fn default() -> Self {
        Self {
            render_type: RenderOptions::Solid,
            color_scheme: ColorScheme::Solid(Color::WHITE),
        }
    }
}

// adding this for integration with Bevy
pub struct StructurePlugin {
    initial_files: Vec<(PathBuf, StructureSettings)>,
}

impl StructurePlugin {
    pub fn new() -> Self {
        Self {
            initial_files: Vec::new(),
        }
    }
    pub fn with_file<P: Into<PathBuf>>(
        mut self,
        path: P,
        settings: Option<StructureSettings>,
    ) -> Self {
        self.initial_files
            .push((path.into(), settings.unwrap_or_default()));
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
struct StructureFiles(Vec<(PathBuf, StructureSettings)>);

#[derive(Event)]
pub struct LoadProteinEvent(pub PathBuf);

fn load_initial_proteins(
    structure_files: Res<StructureFiles>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (file_path, settings) in &structure_files.0 {
        if !Path::new(file_path).exists() {
            eprintln!("Error: File not found: {:?}", file_path);
            continue; // Skip to the next file
        }

        if let Ok((pdb, _errors)) = pdbtbx::open(
            file_path.to_str().unwrap_or_default(),
            StrictnessLevel::Medium,
        ) {
            let structure = Structure::builder()
                .pdb(pdb)
                .rendertype(settings.render_type.clone())
                .color_scheme(settings.color_scheme.clone())
                .build();
            let mesh = structure.to_mesh();
            println!("Number of verices in the mesh: {}", mesh.count_vertices());
            let mesh_handle = meshes.add(mesh);
            let material = materials.add(StandardMaterial {
                base_color: Color::srgb(0.8, 0.7, 0.6),
                metallic: 0.1,
                perceptual_roughness: 0.5,
                ..default()
            });

            commands.spawn((
                structure,
                PbrBundle {
                    mesh: mesh_handle,
                    material,
                    ..default()
                },
            ));
        }
    }
}
