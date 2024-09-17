//! A simple 3D scene with light shining over a cube sitting on a plane.
use bevy::prelude::*;
use pdbtbx::{StrictnessLevel};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, load_pdb))
        .run();
}

fn setup(
    mut commands: Commands,
) {
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
    let (pdb, _errors) = pdbtbx::open(
            "examples/1fap.cif",
            StrictnessLevel::Medium
        ).unwrap();


    // Create a sphere mesh that will be instanced for each atom

    let sphere_mesh = meshes.add(
        Sphere::default().mesh().uv(32,18)
    );

    // Create a default material
    let material = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.0, 0.0),
        ..default()
    });

    // Iterate through ATOM records and create spheres
    for atom in pdb.atoms() {
        let (x, y, z) = atom.pos();
        let (x, y, z) = (x as f32, y as f32, z as f32);

        // Spawn a PbrBundle for each atom
        commands.spawn(PbrBundle {
            mesh: sphere_mesh.clone(),
            material: material.clone(),
            transform: Transform::from_xyz(x, y, z),
            ..default()
        });
    }
}
