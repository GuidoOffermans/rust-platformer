use bevy::{
    prelude::*,
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
};

use crate::AppState;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app
                .add_system_set(SystemSet::on_enter(AppState::InGame)
                    .with_system(setup))
                .add_system_set(SystemSet::on_update(AppState::InGame)
                    .with_system(counter_system));
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn_bundle(TextBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: "\nFPS: ".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 30.0,
                        color: Color::rgb(0.0, 1.0, 0.0),
                    },
                },
                TextSection {
                    value: "".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 30.0,
                        color: Color::rgb(0.0, 1.0, 1.0),
                    },
                },
            ],
            ..Default::default()
        },
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    })
        .insert(Name::new("FPS counter"))
        .id();
}

fn counter_system(
    diagnostics: Res<Diagnostics>,
    mut query: Query<&mut Text>,
) {
    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(average) = fps.average() {
            for mut text in query.iter_mut() {
                text.sections[1].value = format!("{:.2}", average);
            }
        }
    };
}