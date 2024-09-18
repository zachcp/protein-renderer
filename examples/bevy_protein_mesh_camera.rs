//! A simple 3D scene with light shining over a cube sitting on a plane.
use bevy::prelude::*;
use pdbtbx::{self, StrictnessLevel, PDB};
use protein_renderer_core::{ColorScheme, RenderOptions, Structure};

// adding this for integration with Bevy
pub struct StructurePlugin;

// adding this for integration with Bevy
impl Plugin for StructurePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_proteins)
            .add_systems(Update, (update_protein_meshes, focus_camera_on_proteins));
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(StructurePlugin)
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component)]
struct MainCamera;

fn setup(mut commands: Commands) {
    // Add a camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 50.0, 100.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        MainCamera,
    ));

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

fn load_proteins(mut commands: Commands) {
    // Load your PDB files and create Protein components
    let (pdb, _errors) = pdbtbx::open("examples/1fap.cif", StrictnessLevel::Medium).unwrap();
    let structure = Structure::builder()
        .pdb(pdb)
        .color_scheme(ColorScheme::ByAtomType)
        .build();

    commands.spawn((structure, TransformBundle::default()));
}

fn update_protein_meshes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(Entity, &Structure), (Changed<Structure>, With<Structure>)>,
) {
    for (entity, protein) in query.iter() {
        let mesh = protein.render();
        let mesh_handle = meshes.add(mesh);
        let material = materials.add(StandardMaterial {
            base_color: Color::WHITE,
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