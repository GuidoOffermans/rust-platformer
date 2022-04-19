use bevy::prelude::*;
use crate::AppState;

pub struct AsciiPlugin;

impl Plugin for AsciiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(AppState::Menu)
                .with_system(load_ascii));
    }
}

pub struct AsciiSheet(Handle<TextureAtlas>);

pub fn spawn_ascii_sprite(
    commands: &mut Commands,
    ascii: &AsciiSheet,
    index: usize,
    color: Color,
    translation: Vec3,
) -> Entity {
    let mut sprite = TextureAtlasSprite::new(index);
    sprite.custom_size = Some(Vec2::new(50.0 * 2.5, 37.0 * 2.5));
    sprite.color = color;

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite,
            texture_atlas: ascii.0.clone(),
            transform: Transform {
                translation,
                ..Default::default()
            },
            ..Default::default()
        }).id()
}

fn load_ascii(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let image = assets.load("adventurer.png");
    let atlas = TextureAtlas::from_grid_with_padding(
        image,
        Vec2::new(50.0, 37.0),
        7,
        16,
        Vec2::splat(0.0),
    );

    let atlas_handle = texture_atlases.add(atlas);

    commands.insert_resource(AsciiSheet(atlas_handle));
}