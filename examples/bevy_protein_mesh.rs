//! A simple 3D scene with light shining over a cube sitting on a plane.
use bevy::prelude::*;
use pdbtbx::{self, StrictnessLevel, PDB};
use protein_renderer_core::{ColorScheme, RenderOptions, Structure};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, load_pdb))
        .run();
}

fn setup(mut commands: Commands) {
    // Add a camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 100.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Add a light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}

fn load_pdb(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Load the PDB file
    let (mut pdb, _errors) = pdbtbx::open("examples/1fap.cif", StrictnessLevel::Medium).unwrap();
    let structure = Structure::builder()
        .pdb(pdb)
        .color_scheme(ColorScheme::ByAtomType)
        .build();

    let mesh = structure.render();
    let mesh_handle = meshes.add(mesh);

    // Note: why do I need multiple materials?
    let material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        ..default()
    });

    // Spawn a PbrBundle for each atom
    commands.spawn(PbrBundle {
        mesh: mesh_handle,
        material: material,
        ..default()
    });
}
