use bevy::prelude::*;

use crate::game::ldtk::{helpers, LayerInfo, TILE_SCALE};
use crate::TileCollider;

#[derive(Bundle)]
struct CollideIntTileBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    collider: TileCollider,
}

pub fn handle_int_grid(
    commands: &mut Commands,
    layer_info: LayerInfo,
    int_tiles: Vec<i64>,
    int_grid_materials: Vec<Color>,
) {
    let mut tiles = Vec::new();
    let mut collide_tiles = Vec::new();

    for (index, tile) in int_tiles.iter().enumerate() {
        let tiles_per_row = layer_info.px_width / (layer_info.grid_cell_size as f32 * TILE_SCALE);

        let x = ((index as f32 % tiles_per_row + 1.0) * layer_info.grid_cell_size as f32) - layer_info.grid_cell_size as f32;
        let y = (index as f32 / tiles_per_row) as i32 * layer_info.grid_cell_size;

        if *tile == 1 {
            let new_tile = commands.spawn_bundle(CollideIntTileBundle {
                sprite_bundle: SpriteBundle {
                    sprite: Sprite {
                        custom_size: Option::from(
                            Vec2::new(
                                layer_info.grid_cell_size as f32 * TILE_SCALE,
                                layer_info.grid_cell_size as f32 * TILE_SCALE,
                            )),
                        // color: int_grid_materials[*tile as usize].clone(),
                        color: Color::rgba(0.0, 0.0, 0.0, 0.0),
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: helpers::convert_to_world_coordinates(
                            layer_info.px_width,
                            layer_info.px_height,
                            layer_info.grid_cell_size,
                            TILE_SCALE,
                            x as i32,
                            y as i32,
                            layer_info.z_index,
                        ),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                collider: TileCollider,
            }).id();
            collide_tiles.push(new_tile);
        } else {
            let new_tile = commands.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Option::from(
                        Vec2::new(
                            layer_info.grid_cell_size as f32 * TILE_SCALE,
                            layer_info.grid_cell_size as f32 * TILE_SCALE,
                        )),
                    color: int_grid_materials[*tile as usize].clone(),
                    ..Default::default()
                },
                transform: Transform {
                    translation: helpers::convert_to_world_coordinates(
                        layer_info.px_width,
                        layer_info.px_height,
                        layer_info.grid_cell_size,
                        TILE_SCALE,
                        x as i32,
                        y as i32,
                        layer_info.z_index,
                    ),
                    ..Default::default()
                },
                ..Default::default()
            }).id();
            tiles.push(new_tile);
        }
    }

    commands.spawn()
        .insert(Name::new("IntMap"))
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .push_children(&tiles)
        .push_children(&collide_tiles);
}
