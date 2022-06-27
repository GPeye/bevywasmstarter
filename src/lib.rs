use bevy::prelude::*;
use bevy::window::{WindowMode, WindowResizeConstraints, WindowResized};
use bevy_inspector_egui::{WorldInspectorParams, WorldInspectorPlugin};
use wasm_bindgen::prelude::*;

mod game;
use game::{GamePlugin, Log};

#[wasm_bindgen(start)]
pub fn start() {
    Log("Starting Wasm Bevy Template!");

    App::new()
        .insert_resource(WindowDescriptor {
            title: "Template!".to_string(),
            width: 1024.0,
            height: 768.0,
            mode: WindowMode::Windowed,
            resizable: false,
            ..default()
        })
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_camera)
        .insert_resource(WorldInspectorParams {
            enabled: false,
            ..Default::default()
        })
        .add_plugin(WorldInspectorPlugin::new())
        .add_system(toggle_inspector)
        .add_plugin(GamePlugin)
        .run();
}

fn toggle_inspector(
    input: ResMut<Input<KeyCode>>,
    mut window_params: ResMut<WorldInspectorParams>,
) {
    if input.just_pressed(KeyCode::Grave) {
        window_params.enabled = !window_params.enabled
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
