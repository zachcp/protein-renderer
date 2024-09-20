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
    app.send_event(LoadProteinEvent(file_data.to_vec()));
}

fn player_level_up(mut ev_levelup: EventWriter<LevelUpEvent>, query: Query<(Entity, &PlayerXp)>) {
    for (entity, xp) in query.iter() {
        if xp.0 > 1000 {
            ev_levelup.send(LevelUpEvent(entity));
        }
    }
}

fn handle_protein_upload(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut upload_events: EventReader<LoadProteinEvent>,
) {
    for event in upload_events.read() {
        let LoadProteinEvent(file_data) = event;
        println!("Received protein data, length: {}", file_data.len());

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
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
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

#[cfg(target_arch = "wasm32")]
/// WASM Part.
#[derive(Resource)]
pub struct ReceiverResource<T> {
    pub rx: async_std::channel::Receiver<T>,
}

#[cfg(target_arch = "wasm32")]
fn listen_js_escher(
    receiver: Res<ReceiverResource<EscherMap>>,
    mut escher_asset: ResMut<Assets<EscherMap>>,
    mut escher_resource: ResMut<MapState>,
) {
    if let Ok(escher_map) = receiver.rx.try_recv() {
        escher_resource.escher_map = escher_asset.add(escher_map);
        escher_resource.loaded = false;
    }
}

#[cfg(target_arch = "wasm32")]
fn listen_js_data(
    receiver: Res<ReceiverResource<Data>>,
    mut data_asset: ResMut<Assets<Data>>,
    mut data_resource: ResMut<ReactionState>,
) {
    if let Ok(escher_map) = receiver.rx.try_recv() {
        data_resource.reaction_data = Some(data_asset.add(escher_map));
        data_resource.loaded = false;
    }
}

#[cfg(target_arch = "wasm32")]
fn listen_js_info(receiver: Res<ReceiverResource<&'static str>>, mut info_box: ResMut<Info>) {}
