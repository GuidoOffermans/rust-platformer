use bevy::prelude::*;

mod player;
mod ldtk;
mod ascii;
mod inspector;

use crate::AppState;
use crate::debug::DebugPlugin;
use crate::game::ascii::AsciiPlugin;
use crate::game::inspector::InspectorPlugin;
use crate::game::ldtk::LDTKPlugin;
use crate::game::player::PlayerPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(AppState::InGame)
                .with_system(spawn_camera))
            .add_system_set(SystemSet::on_update(AppState::InGame))
            .add_plugin(InspectorPlugin)
            .add_plugin(DebugPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(AsciiPlugin)
            .add_plugin(LDTKPlugin);
    }
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.transform = Transform::from_translation(Vec3::new(0.0, 0.0, 100.0));
    commands.spawn().insert_bundle(camera);
}



