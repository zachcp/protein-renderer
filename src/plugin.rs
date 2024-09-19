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

fn load_initial_proteins(mut commands: Commands, structure_files: Res<StructureFiles>) {
    for file_path in &structure_files.0 {
        load_protein(&mut commands, file_path);
    }
}

fn load_protein(commands: &mut Commands, file_path: &PathBuf) {
    if let Ok((pdb, _errors)) = pdbtbx::open(
        file_path.to_str().unwrap_or_default(),
        StrictnessLevel::Medium,
    ) {
        let structure = Structure::builder()
            .pdb(pdb)
            .color_scheme(ColorScheme::ByAtomType)
            .build();

        commands.spawn((structure, TransformBundle::default()));
    } else {
        error!("Failed to load protein file: {:?}", file_path);
    }
}

fn handle_load_protein_event(
    mut commands: Commands,
    mut ev_load_protein: EventReader<LoadProteinEvent>,
) {
    for ev in ev_load_protein.read() {
        load_protein(&mut commands, &ev.0);
    }
}
