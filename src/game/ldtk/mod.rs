use std::collections::HashMap;
use bevy::prelude::*;

use ldtk_rust::{Project};
use crate::AppState;

pub(crate) mod tiles;
mod entities;
mod helpers;
mod int_grid;

const LDTK_FILE_PATH: &str = "assets/test.ldtk";
// const BASE_DIR: &str = "assets/";
// const PROJECT_FILE: &str = "SeparateLevelFiles.ldtk";
pub const TILE_SCALE: f32 = 2.5;

pub struct LDTKPlugin;

impl Plugin for LDTKPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(AppState::InGame)
                .with_system(setup))
            .add_system_set(SystemSet::on_update(AppState::InGame)
                .with_system(update));
    }
}

struct Map {
    ldtk_file: Project,
    redraw: bool,
    current_level: usize,
}

#[derive(Clone)]
pub struct VisualAssets {
    int_grid_materials: HashMap<i32, Vec<Color>>,
    sprite_sheets: HashMap<i32, Handle<TextureAtlas>>,
    entity_materials: HashMap<i32, Handle<ColorMaterial>>,
}

// #[derive(Clone, Copy)]
// pub struct LevelInfo {
//     current_level_uid: i64,
//     current_level_path: Option<String>,
//     current_level_data: Option<Level>,
// }

#[derive(Clone, Copy)]
pub struct LayerInfo {
    grid_width: i32,
    _grid_height: i32,
    grid_cell_size: i32,
    z_index: i32,
    px_width: f32,
    px_height: f32,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // let mut current_level_path: Option<String> = None;
    // let mut current_level_data: Option<Level> = None;

    // let project = Project::load_project(BASE_DIR.to_owned() + PROJECT_FILE);

    // match current_level_path {
    //     Some(t) => current_level_data = Some(Level::new(BASE_DIR.to_owned() + &t)),
    //     None => println!("that level ID is not correct."),
    // };

    // // print some data
    // match current_level_data {
    //     Some(t) => println!(
    //         "Level {} has {} layer instaces.",
    //         current_level_uid,
    //         t.layer_instances.unwrap().len()
    //     ),
    //     None => println!("no level data"),
    // }


    let map = Map {
        ldtk_file: Project::new(LDTK_FILE_PATH.to_string()),
        redraw: true,
        current_level: 0,
    };

    let mut visual_assets = VisualAssets {
        int_grid_materials: HashMap::new(),
        sprite_sheets: HashMap::new(),
        entity_materials: HashMap::new(),
    };

    // Load tilesets.
    for tileset in map.ldtk_file.defs.tilesets.iter() {
        let texture_handle = asset_server.load(&tileset.rel_path[..]);

        let texture_atlas = TextureAtlas::from_grid(
            texture_handle,
            Vec2::new(tileset.tile_grid_size as f32, tileset.tile_grid_size as f32),
            (tileset.px_wid / tileset.tile_grid_size) as usize,
            (tileset.px_hei / tileset.tile_grid_size) as usize,
        );

        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        visual_assets
            .sprite_sheets
            .insert(tileset.uid as i32, texture_atlas_handle);
    }

    for layer in map
        .ldtk_file
        .defs
        .layers
        .iter()
        .filter(|f| match f.purple_type {
            ldtk_rust::Type::IntGrid => true,
            _ => false,
        })
    {
        let mut colors = Vec::new();
        colors.push(Color::rgba(0.0, 0.0, 0.0, 0.0));

        for i in layer.int_grid_values.iter() {
            println!("I: {}, C: {}", i.value, i.color);
            let clr = match Color::hex(&i.color[1..]) {
                Ok(t) => t,
                Err(e) => {
                    println!("Error: {:?}", e);
                    Color::BLUE
                }
            };
            colors.push(clr);
        }
        println!("Colors: {:?}", colors);
        visual_assets
            .int_grid_materials
            .insert(layer.uid as i32, colors);
    }

    // Load enities.
    for entity in map.ldtk_file.defs.entities.iter() {
        let color = match Color::hex(&entity.color.clone()[1..]) {
            Ok(t) => t,
            Err(e) => {
                println!("Error: {:?}", e);
                Color::BLUE
            }
        };
        let color_material = materials.add(ColorMaterial::from(color));

        visual_assets
            .entity_materials
            .insert(entity.uid as i32, color_material);
    }

    commands.insert_resource(map);
    commands.insert_resource(visual_assets);
}

fn update(
    mut commands: Commands,
    mut map: ResMut<Map>,
    visual_assets: Res<VisualAssets>,
) {
    if !map.redraw {
        return;
    }

    commands.insert_resource(ClearColor(
        Color::hex(&map.ldtk_file.levels[0].bg_color[1..]).unwrap(),
    ));

    for (idx, layer) in map.ldtk_file.levels[map.current_level]
        .layer_instances
        .as_ref()
        .unwrap()
        .iter()
        .enumerate()
        .rev()
    {
        let tileset_uid = layer.tileset_def_uid.unwrap_or(-1) as i32;
        println!("tileset id: {}", &tileset_uid);
        let layer_uid = layer.layer_def_uid as i32;

        let layer_info = LayerInfo {
            grid_width: layer.c_wid as i32,
            _grid_height: layer.c_hei as i32,
            grid_cell_size: layer.grid_size as i32,
            z_index: 50 - idx as i32,
            px_width: layer.c_wid as f32 * (layer.grid_size as f32 * TILE_SCALE),
            px_height: layer.c_hei as f32 * (layer.grid_size as f32 * TILE_SCALE),
        };
        println!("Layer: {:?}", layer_uid);
        match &layer.layer_instance_type[..] {
            "Tiles" => {
                println!("Generating Tile Layer: {}", layer.identifier);
                tiles::display_tiles(
                    &mut commands,
                    layer_info,
                    &layer.grid_tiles,
                    visual_assets.sprite_sheets[&tileset_uid].clone(),
                );
            }
            "IntGrid" => {
                println!("Generating IntGrid Layer: {}", layer.identifier);
                int_grid::handle_int_grid(
                    &mut commands,
                    layer_info,
                    layer.int_grid_csv.clone(),
                    visual_assets.int_grid_materials[&layer_uid].clone(),
                );
            }
            "Entities" => {
                println!("Generating Entities Layer: {}", layer.identifier);
                entities::display_entities(
                    &mut commands,
                    &layer.entity_instances,
                    &map.ldtk_file.defs,
                    layer_info,
                    visual_assets.clone(),
                );
            }
            _ => {
                println!("Not Implemented: {}", layer.identifier);
            }
        }
    }

    // Whew, we've draw everyting so update the Map instance so we don't do it every game loop.
    map.redraw = false;
}

