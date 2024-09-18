//! A simple 3D scene with light shining over a cube sitting on a plane.
use bevy::prelude::*;
use pdbtbx::{self, StrictnessLevel, PDB};
// use protein_renderer_structure;
// use protein_renderer_structure::representations::pdb_to_mesh;
// use protein_renderer_core;
// use protein_renderer_core::traits::renderable::{RenderOption, Renderable};
use protein_renderer_core::{RenderOptions, Renderable};

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

    // let sphere_mesh = meshes.add(Sphere::default().mesh().uv(32, 18));
    //
    // let color_func = |atom: &pdbtbx::Atom| {
    //     match atom.element().expect("expect atom").symbol() {
    //         "C" => Color::srgb(0.5, 0.5, 0.5), // Carbon: Gray
    //         "N" => Color::srgb(0.0, 0.0, 1.0), // Nitrogen: Blue
    //         "O" => Color::srgb(1.0, 0.0, 0.0), // Oxygen: Red
    //         "S" => Color::srgb(1.0, 1.0, 0.0), // Sulfur: Yellow
    //         _ => Color::srgb(1.0, 1.0, 1.0),   // Other: White
    //     }
    // };

    // Create a default material
    let material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        ..default()
    });

    // Iterate through ATOM records and create spheres
    for atom in pdb.atoms() {
        let (x, y, z) = atom.pos();
        let (x, y, z) = (x as f32, y as f32, z as f32);
    }

    // let mesh = pdb_to_mesh(&pdb, color_func);
    let mesh = pdb.generate_mesh(RenderOptions::Solid);
    let mesh_handle = meshes.add(mesh);

    // Spawn a PbrBundle for each atom
    commands.spawn(PbrBundle {
        mesh: mesh_handle,
        material: material,
        ..default()
    });
}
