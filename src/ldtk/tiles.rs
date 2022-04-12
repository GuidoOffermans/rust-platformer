use std::collections::HashMap;
use bevy::{prelude::*};
use ldtk_rust::{EntityInstance, Project, TileInstance};

use crate::ldtk::{LayerInfo, TILE_SCALE};
use crate::ldtk::helpers;

pub fn display_tiles(
    commands: &mut Commands,
    layer_info: LayerInfo,
    tiles: &Vec<TileInstance>,
    atlas_handle: Handle<TextureAtlas>,
) {
    let mut new_tiles = Vec::new();

    for tile in tiles.iter() {
        let mut flip_x = false;
        let mut flip_y = false;
        match tile.f {
            1 => flip_x = true,
            2 => flip_y = true,
            3 => {
                flip_x = true;
                flip_y = true
            }
            _ => (),
        }

        let new_tile = commands.spawn_bundle(SpriteSheetBundle {
            transform: Transform {
                translation: helpers::convert_to_world_coordinates(
                    layer_info.px_width,
                    layer_info.px_height,
                    layer_info.grid_cell_size,
                    TILE_SCALE,
                    tile.px[0] as i32,
                    tile.px[1] as i32,
                    layer_info.z_index,
                ),
                rotation: helpers::flip(flip_x, flip_y),
                scale: Vec3::splat(TILE_SCALE),
            },
            sprite: TextureAtlasSprite::new(tile.t as usize),
            texture_atlas: atlas_handle.clone(),
            ..Default::default()
        }).id();
        new_tiles.push(new_tile);
    }
    commands.spawn()
        .insert(Name::new("Map"))
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .push_children(&new_tiles);
}