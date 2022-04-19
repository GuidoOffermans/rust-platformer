use bevy::{
    prelude::*,
};
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};
use crate::game::player::Player;

pub struct InspectorPlugin;

impl Plugin for InspectorPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app
                .add_plugin(WorldInspectorPlugin::new())
                .register_inspectable::<Player>();
        }
    }
}
