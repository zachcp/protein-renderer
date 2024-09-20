use bevy::prelude::*;
use pdbtbx::{self, StrictnessLevel};
use protein_renderer::{Structure, StructurePlugin};
use std::io::BufReader;
use std::io::Cursor;
use wasm_bindgen::prelude::*;

// Event to trigger protein loading
#[derive(Event)]
pub struct LoadProteinEvent(pub Vec<u8>);

#[wasm_bindgen]
pub fn upload_protein_file(file_data: &[u8]) {
    // Get the Bevy app instance and send the event
    let world = &mut World::default();
    world.send_event(LoadProteinEvent(file_data.to_vec()));
}

fn handle_protein_upload(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut upload_events: EventReader<LoadProteinEvent>,
) {
    for LoadProteinEvent(file_data) in upload_events.read() {
        let cursor = Cursor::new(file_data);
        let buf_reader = BufReader::new(cursor);
        let (pdb, error) = pdbtbx::open_raw(buf_reader, StrictnessLevel::Medium).unwrap();
        // if let Some(error) = error {
        //     println!("Warning: PDB parsing had errors: {:?}", error);
        // }
        let structure = Structure::builder().pdb(pdb).build();
        let pbr = structure.to_pbr(&mut meshes, &mut materials);
        commands.spawn((structure, pbr));
    }
}

#[wasm_bindgen]
pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(StructurePlugin::new())
        .add_event::<LoadProteinEvent>()
        .add_systems(Update, handle_protein_upload)
        .add_systems(Startup, setup_scene)
        .run();
}

fn setup_scene(mut commands: Commands) {
    // Add a camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
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
