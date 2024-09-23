use bevy::prelude::*;
use bevy_file_dialog::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct GameApp {
    app: App,
}

#[wasm_bindgen]
impl GameApp {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut app = App::new();
        app.add_plugins(DefaultPlugins)
            .add_plugins(FileDialogPlugin::new().with_load_file::<PrintFilePath>())
            .add_systems(Update, (file_picked, handle_pick_request))
            .add_event::<PickFileEvent>();

        Self { app }
    }

    #[wasm_bindgen]
    pub fn run(&mut self) {
        self.app.update();
    }

    #[wasm_bindgen]
    pub fn pick_file(&mut self, file_name: String) {
        self.app.world_mut().send_event(PickFileEvent(file_name));
    }
}

struct PrintFilePath;

#[derive(Event)]
struct PickFileEvent(String);

fn handle_pick_request(mut commands: Commands, mut events: EventReader<PickFileEvent>) {
    for ev in events.read() {
        // commands.dialog().load_file::<PrintFilePath>();
        web_sys::console::log_1(&format!("File picked: {}", ev.0).into());
    }
}

// fn pick_file(mut commands: Commands, mut events: EventReader<PickFileEvent>) {
//     for _ in events.read() {
//         commands.dialog().load_file::<PrintFilePath>();
//     }
// }

fn file_picked(mut ev_picked: EventReader<DialogFileLoaded<PrintFilePath>>) {
    for ev in ev_picked.read() {
        eprintln!("File picked, path {:?}", ev.file_name);
        web_sys::console::log_1(&format!("File picked, path {:?}", ev.file_name).into());
    }
}
