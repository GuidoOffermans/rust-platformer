use bevy::{
    prelude::*,
    render::camera::ScalingMode,
    window::WindowMode,
};

mod player;
mod debug;
mod ascii;
mod tilemap;
mod tiles;
mod ldtk;

use crate::ascii::AsciiPlugin;
use crate::player::PlayerPlugin;
use crate::debug::DebugPlugin;
use crate::tilemap::TileMapPlugin;
use crate::tiles::TilesPlugin;
use crate::ldtk::LDTKPlugin;

pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const TILE_SIZE: f32 = 2.5;

fn main() {
    App::new()
        .insert_resource(Window::new())
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(AsciiPlugin)
        // .add_plugin(TilesPlugin)
        .add_plugin(LDTKPlugin)
        .add_startup_system(spawn_camera)
        .run();
}

struct Window {}

impl Window {
    fn new() -> WindowDescriptor {
        let height: f32 = 600.0;
        WindowDescriptor {
            width: height * RESOLUTION,
            height,
            title: "Platformer".to_string(),
            vsync: false,
            mode: WindowMode::Windowed,
            resizable: false,
            ..Default::default()
        }
    }
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.transform = Transform::from_translation(Vec3::new(0.0, 0.0, 100.0));
    commands.spawn().insert_bundle(camera);
}
