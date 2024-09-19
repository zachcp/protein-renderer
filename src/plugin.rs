use super::{ColorScheme, Structure};
use bevy::prelude::*;
use pdbtbx::StrictnessLevel;
// adding this for integration with Bevy
pub struct StructurePlugin;

// adding this for integration with Bevy
impl Plugin for StructurePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_proteins);
    }
}

fn load_proteins(mut commands: Commands) {
    // Load your PDB files and create Protein components
    let (pdb, _errors) = pdbtbx::open("examples/1fap.cif", StrictnessLevel::Medium).unwrap();
    let structure = Structure::builder()
        .pdb(pdb)
        .color_scheme(ColorScheme::ByAtomType)
        .build();

    commands.spawn((structure, TransformBundle::default()));
}
