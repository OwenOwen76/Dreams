mod camera;
mod characters;
mod collision;
mod debug;
mod map;
mod npc;
mod pathfinding;
mod state;

use bevy::{
    prelude::*,
    window::{Window, WindowPlugin, WindowResolution},
};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Dreams".into(),
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(state::StatePlugin)
        .add_plugins(map::MapPlugin)
        .add_plugins(characters::CharactersPlugin)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(debug::DebugPlugin)
        .run();
}
