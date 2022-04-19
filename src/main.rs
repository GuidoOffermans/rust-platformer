use bevy::{
    prelude::*,
    window::WindowMode,
    diagnostic::FrameTimeDiagnosticsPlugin,
    window::PresentMode
};

mod debug;
mod menu;
mod game;

use crate::menu::MenuPlugin;
use crate::game::GamePlugin;

pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const TILE_SCALE: f32 = 2.5;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    Menu,
    InGame,
}

fn main() {
    App::new()
        .insert_resource(Window::new(600.0))
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_state(AppState::Menu)
        .add_plugin(MenuPlugin)
        .add_plugin(GamePlugin)
        .run();
}

#[derive(Component)]
pub struct TileCollider;

struct Window {}

impl Window {
    fn new(height: f32) -> WindowDescriptor {
        WindowDescriptor {
            width: height * RESOLUTION,
            height,
            title: "Platformer".to_string(),
            present_mode: PresentMode::Mailbox,
            mode: WindowMode::Windowed,
            resizable: false,
            ..Default::default()
        }
    }
}
