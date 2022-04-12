use std::collections::HashMap;
use bevy::{prelude::*};
use ldtk_rust::{Definitions, EntityDefinition, EntityInstance, Project, TileInstance};

use crate::ldtk::{LayerInfo, TILE_SCALE, VisualAssets};
use crate::ldtk::helpers;

#[derive(Copy, Clone)]
struct ExtraEntDefs {
    __tile_id: i32,
    __width: i32,
    __height: i32,
    __scale: f32,
}

impl ExtraEntDefs {
    fn new() -> Self {
        Self {
            __tile_id: 0,
            __width: 0,
            __height: 0,
            __scale: 1.0,
        }
    }
}

pub fn display_entities(
    commands: &mut Commands,
    entities: &Vec<EntityInstance>,
    definitions: &Definitions,
    layer_info: LayerInfo,
    visual_assets: VisualAssets
) {
    for entity in entities.iter() {
        let mut extra_ent_defs = ExtraEntDefs::new();
        for ent in definitions.entities.iter() {
            if ent.uid == entity.def_uid {
                extra_ent_defs.__tile_id = 0;
                extra_ent_defs.__width = ent.width as i32;
                extra_ent_defs.__height = ent.height as i32;
            }
            match ent.render_mode {
                ldtk_rust::RenderMode::Tile => {
                    extra_ent_defs.__tile_id = ent.tile_id.unwrap() as i32;
                    for ts in definitions.tilesets.iter() {
                        if ts.uid == ent.tileset_id.unwrap() {
                            extra_ent_defs.__scale =
                                ent.width as f32 / ts.tile_grid_size as f32;
                        }
                    }
                }
                _ => (),
            }
        }

        display_entity(
            commands,
            entity,
            &extra_ent_defs,
            layer_info,
            visual_assets.clone(),
        );
    }
}

fn display_entity(
    commands: &mut Commands,
    entity: &EntityInstance,
    extra_ent_defs: &ExtraEntDefs,
    layer_info: LayerInfo,
    visual_assets: VisualAssets,
) {
    match &entity.tile {
        Some(tileset_entity) => {
            // process tile asset
            let tileset_uid = tileset_entity.tileset_uid as i32;
            let handle: Handle<TextureAtlas> = visual_assets.spritesheets[&tileset_uid].clone();

            commands.spawn().insert_bundle(SpriteSheetBundle {
                transform: Transform {
                    translation: helpers::convert_to_world_coordinates(
                        layer_info.px_width,
                        layer_info.px_height,
                        extra_ent_defs.__height,
                        TILE_SCALE,
                        entity.grid[0] as i32 * layer_info.grid_cell_size,
                        entity.grid[1] as i32 * layer_info.grid_cell_size,
                        layer_info.z_index,
                    ),
                    scale: Vec3::splat(extra_ent_defs.__scale * TILE_SCALE),
                    ..Default::default()
                },
                sprite: TextureAtlasSprite::new(extra_ent_defs.__tile_id as usize),
                texture_atlas: handle,
                ..Default::default()
            });
        }
        None => {
            // process color shape
            let handle: Handle<ColorMaterial> = visual_assets.entity_materials[&(entity.def_uid as i32)].clone();
            commands
                .spawn()
                .insert_bundle(SpriteBundle {
                    // material: handle,
                    sprite: Sprite {
                        // color: handle,
                        custom_size: Option::from(Vec2::new(extra_ent_defs.__width as f32, extra_ent_defs.__height as f32)),
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: helpers::convert_to_world_coordinates(
                            layer_info.px_width,
                            layer_info.px_height,
                            extra_ent_defs.__height,
                            TILE_SCALE,
                            entity.grid[0] as i32 * layer_info.grid_cell_size,
                            entity.grid[1] as i32 * layer_info.grid_cell_size,
                            layer_info.z_index,
                        ),
                        scale: Vec3::splat(TILE_SCALE),
                        ..Default::default()
                    },
                    ..Default::default()
                });
        }
    }
}
