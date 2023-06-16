pub mod components;
mod shaders;
mod systems;

use bevy::prelude::*;
use systems::*;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_startup_system(setup)
        .add_system(movement_system)
        .add_system(look_system)
        .add_system(grab_mouse)
        .run();
}
