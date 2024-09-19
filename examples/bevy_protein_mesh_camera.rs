//! A simple 3D scene with light shining over a cube sitting on a plane.
use bevy::prelude::*;
use pdbtbx::{self, StrictnessLevel};
use protein_renderer::{ColorScheme, Structure, StructurePlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(StructurePlugin) // <--- put any special magic here
        .add_systems(Startup, setup)
        .add_systems(Update, (update_protein_meshes, focus_camera_on_proteins))
        .run();
}

#[derive(Component)]
struct MainCamera;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Add a camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 50.0, 100.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        MainCamera,
    ));

    // Key Light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::srgb(1.0, 0.9, 0.9),
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.5, 0.5, 0.0)),
        ..default()
    });

    // Fill Light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::srgb(0.8, 0.8, 1.0),
            illuminance: 5000.0,
            shadows_enabled: false,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, 0.5, -0.5, 0.0)),
        ..default()
    });

    // Back Light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::srgb(0.9, 0.9, 1.0),
            illuminance: 3000.0,
            shadows_enabled: false,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(
            EulerRot::XYZ,
            0.0,
            std::f32::consts::PI,
            0.0,
        )),
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

    // Spot light
    commands.spawn(SpotLightBundle {
        spot_light: SpotLight {
            intensity: 10000.0,
            color: Color::srgb(0.8, 1.0, 0.8),
            shadows_enabled: true,
            outer_angle: 0.6,
            ..default()
        },
        transform: Transform::from_xyz(-4.0, 5.0, -4.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn update_protein_meshes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(Entity, &Structure), (Changed<Structure>, With<Structure>)>,
) {
    println!("I'm in the update_protein_mesh function!");
    println!("Total entities with Structure: {}", query.iter().count());
    for (entity, protein) in query.iter() {
        println!("Working on {:}", entity);
        let mesh = protein.to_mesh();
        let mesh_handle = meshes.add(mesh);
        let material = materials.add(StandardMaterial {
            base_color: Color::WHITE,
            perceptual_roughness: 0.9,
            ..default()
        });

        commands.entity(entity).insert(PbrBundle {
            mesh: mesh_handle,
            material,
            ..default()
        });
    }
}

fn focus_camera_on_proteins(
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<Structure>)>,
    protein_query: Query<&Transform, With<Structure>>,
) {
    if let Ok(mut camera_transform) = camera_query.get_single_mut() {
        if let Some(center) = calculate_center_of_proteins(&protein_query) {
            let camera_position = center + Vec3::new(0.0, 50.0, 100.0);
            camera_transform.translation = camera_position;
            camera_transform.look_at(center, Vec3::Y);
        }
    }
}

fn calculate_center_of_proteins(
    protein_query: &Query<&Transform, With<Structure>>,
) -> Option<Vec3> {
    let mut total = Vec3::ZERO;
    let mut count = 0;
    for transform in protein_query.iter() {
        total += transform.translation;
        count += 1;
    }
    if count > 0 {
        Some(total / count as f32)
    } else {
        None
    }
}
