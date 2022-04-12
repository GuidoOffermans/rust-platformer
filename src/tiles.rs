use std::collections::HashMap;
use bevy::{prelude::*};
use ldtk_rust::{EntityInstance, Project, TileInstance};

const LDTK_FILE_PATH: &str = "assets/metroid.ldtk";
const TILE_SCALE: f32 = 2.5;

pub struct TilesPlugin;

impl Plugin for TilesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup.system())
            .add_system(update.system());
    }
}

struct Map {
    ldtk_file: Project,
    redraw: bool,
    current_level: usize,
}

#[derive(Clone)]
struct VisualAssets {
    int_grid_materials: HashMap<i32, Vec<Handle<ColorMaterial>>>,
    spritesheets: HashMap<i32, Handle<TextureAtlas>>,
    entity_materials: HashMap<i32, Handle<ColorMaterial>>,
}

// storage for layer info as we loop through tiles
#[derive(Clone, Copy)]
struct LayerInfo {
    grid_width: i32,
    _grid_height: i32,
    grid_cell_size: i32,
    z_index: i32,
    px_width: f32,
    px_height: f32,
}

// The LDtk JSON is organized in two main sections, the "defs"
// object define things and the "levels" object includes the
// level information. Most users can ignore the "defs" object,
// but if you want something from it, here's one way to do it.
#[derive(Copy, Clone)]
struct ExtraEntDefs {
    __tile_id: i32,
    __width: i32,
    __height: i32,
    __scale: f32,
}

// implement a new() method
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

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let map = Map {
        ldtk_file: Project::new(LDTK_FILE_PATH.to_string()),
        redraw: true,
        current_level: 0,
    };

    println!("{}", "here");

    let mut visual_assets = VisualAssets {
        int_grid_materials: HashMap::new(),
        spritesheets: HashMap::new(),
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
            .spritesheets
            .insert(tileset.uid as i32, texture_atlas_handle);
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

    // For the current level, loop through the Layer Instances and start spawning
    // tiles. These Layer Instances can be one of four different kinds of layers:
    // IntGrid, Entities, Tiles or AutoLayer. Tiles and AutoLayers are easy: you
    // always iterate through Tile Instances to do your work. IntGrids may or may
    // not use a tileset (they can use flat colors). If they have tiles, you handle
    // them just like Tiles and AutLayers (but you have to check for a tileset first).
    // For entities you iterate through Entity Instances instead of Tile Instances.
    //
    // Your game only needs to handle the kinds of layers you want to use in LDtk.
    // Here I try to organize the code to accommodate all the options, but then
    // implement some simplistic code for the ones that use actual tilesets.
    //
    // Using .rev() allows us to handle things "bottom to top" and makes sorting
    // on the z-axis easier to reason about.
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

        // Multiply the grid size by the tile size and our scaling constant
        // to calculate the total width and height of our layer. For depth
        // we pick a starting point and add the loop index so we always draw
        // tiles on top of previous iterations. We do all this in a struct
        // instance so we can easily pass it around to functions later.
        let layer_info = LayerInfo {
            grid_width: layer.c_wid as i32,
            _grid_height: layer.c_hei as i32,
            grid_cell_size: layer.grid_size as i32,
            z_index: 50 - idx as i32,
            px_width: layer.c_wid as f32 * (layer.grid_size as f32 * TILE_SCALE),
            px_height: layer.c_hei as f32 * (layer.grid_size as f32 * TILE_SCALE),
        };

        match &layer.layer_instance_type[..] {
            "Tiles" => {
                println!("Generating Tile Layer: {}", layer.identifier);
                display_tiles(
                    layer_info,
                    &layer.grid_tiles,
                    &mut commands,
                    visual_assets.spritesheets[&tileset_uid].clone(),
                );
            }
            // "AutoLayer" => {
            //     println!("Generating AutoTile Layer: {}", layer.identifier);
            //     for tile in layer.auto_layer_tiles.iter() {
            //         display_tile(
            //             layer_info,
            //             tile,
            //             &mut commands,
            //             visual_assets.spritesheets[&tileset_uid].clone(),
            //         );
            //     }
            // }
            // "IntGrid" => {
            //     match layer.tileset_def_uid {
            //         Some(i) => {
            //             // we have tiles, so handle just like Tiles and AutoLayers
            //             println!("Generating IntGrid Layer w/ Tiles: {}", layer.identifier);
            //             let i = i as i32;
            //             for tile in layer.auto_layer_tiles.iter() {
            //                 display_tile(
            //                     layer_info,
            //                     tile,
            //                     &mut commands,
            //                     visual_assets.spritesheets[&i].clone(),
            //                 );
            //             }
            //         }
            //         None => {
            //             // we do NOT have a corresponding tileset, so we need to use
            //             // the color values to represent the level visually.
            //             println!(
            //                 "Generating IntGrid Layer w/ Color Materials: {}",
            //                 layer.identifier
            //             );
            //             for tile in layer.int_grid_csv.iter() {
            //                 display_color(
            //                     layer_info,
            //                     tile,
            //                     &mut commands,
            //                     visual_assets.int_grid_materials[&layer_uid][*tile as usize]
            //                         .clone(),
            //                 )
            //             }
            //         }
            //     }
            // }
            "Entities" => {
                println!("Generating Entities Layer: {}", layer.identifier);
                // Entities reference their tiles and colors within the instances
                for entity in layer.entity_instances.iter() {
                    // we need some extra fields from the defs section of the
                    // JSON that aren't included in the entity instances.
                    let mut extra_ent_defs = ExtraEntDefs::new();
                    for ent in map.ldtk_file.defs.entities.iter() {
                        if ent.uid == entity.def_uid {
                            extra_ent_defs.__tile_id = 0;
                            extra_ent_defs.__width = ent.width as i32;
                            extra_ent_defs.__height = ent.height as i32;
                        }
                        match ent.render_mode {
                            ldtk_rust::RenderMode::Tile => {
                                extra_ent_defs.__tile_id = ent.tile_id.unwrap() as i32;
                                for ts in map.ldtk_file.defs.tilesets.iter() {
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
                        layer_info,
                        entity,
                        &mut commands,
                        visual_assets.clone(),
                        &extra_ent_defs,
                    );
                }
            }
            _ => {
                println!("Not Implemented: {}", layer.identifier);
            }
        }
    }

    // Whew, we've draw everyting so update the Map instance so we don't do it every game loop.
    map.redraw = false;
}

// Spawn a tile. Check to see if it needs to flip on the x and/or y axis before spawning.
fn display_tile(
    layer_info: LayerInfo,
    tile: &TileInstance,
    commands: &mut Commands,
    handle: Handle<TextureAtlas>,
) {
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

    commands.spawn().insert_bundle(SpriteSheetBundle {
        transform: Transform {
            translation: convert_to_world(
                layer_info.px_width,
                layer_info.px_height,
                layer_info.grid_cell_size,
                TILE_SCALE,
                tile.px[0] as i32,
                tile.px[1] as i32,
                layer_info.z_index,
            ),
            rotation: flip(flip_x, flip_y),
            scale: Vec3::splat(TILE_SCALE),
        },
        sprite: TextureAtlasSprite::new(tile.t as usize),
        texture_atlas: handle,
        ..Default::default()
    });
}

fn display_tiles(
    layer_info: LayerInfo,
    tiles: &Vec<TileInstance>,
    commands: &mut Commands,
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
                translation: convert_to_world(
                    layer_info.px_width,
                    layer_info.px_height,
                    layer_info.grid_cell_size,
                    TILE_SCALE,
                    tile.px[0] as i32,
                    tile.px[1] as i32,
                    layer_info.z_index,
                ),
                rotation: flip(flip_x, flip_y),
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

// spawn your entities.
fn display_entity(
    layer_info: LayerInfo,
    entity: &EntityInstance,
    commands: &mut Commands,
    visual_assets: VisualAssets,
    extra_ent_defs: &ExtraEntDefs,
) {
    match &entity.tile {
        Some(tileset_entity) => {
            // process tile asset
            let tileset_uid = tileset_entity.tileset_uid as i32;
            let handle: Handle<TextureAtlas> = visual_assets.spritesheets[&tileset_uid].clone();

            commands.spawn().insert_bundle(SpriteSheetBundle {
                transform: Transform {
                    translation: convert_to_world(
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
            println!("{}", "oh oh");
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
                        translation: convert_to_world(
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

fn display_color(
    layer_info: LayerInfo,
    tile: &i64,
    commands: &mut Commands,
    handle: Handle<ColorMaterial>,
) {
    let x = *tile as i32 % layer_info.grid_width;
    let y = *tile as i32 / layer_info.grid_width;
    commands.spawn().insert_bundle(SpriteBundle {
        // material: handle,
        sprite: Sprite {
            custom_size: Option::from(Vec2::new(layer_info.grid_cell_size as f32, layer_info.grid_cell_size as f32)),
            ..Default::default()
        },
        transform: Transform {
            translation: convert_to_world(
                layer_info.px_width,
                layer_info.px_height,
                layer_info.grid_cell_size,
                TILE_SCALE,
                x * layer_info.grid_cell_size,
                y as i32 * layer_info.grid_cell_size,
                layer_info.z_index,
            ),
            scale: Vec3::splat(TILE_SCALE),
            ..Default::default()
        },
        ..Default::default()
    });
}

// LDtk provides pixel locations starting in the top left. For Bevy we need to
// flip the Y axis and offset from the center of the screen.
fn convert_to_world(
    width: f32,
    height: f32,
    grid_size: i32,
    scale: f32,
    x: i32,
    y: i32,
    z: i32,
) -> Vec3 {
    let world_x = (x as f32 * scale) + (grid_size as f32 * scale / 2.) - (width / 2.);
    let world_y = -(y as f32 * scale) - (grid_size as f32 * scale / 2.) + (height / 2.);
    let world_z = z as f32;
    Vec3::new(world_x, world_y, world_z)
}

// Bevy doesn't have sprite flipping built in, so if tile needs to flip on either axis, flip it
fn flip(x: bool, y: bool) -> Quat {
    let mut quat1 = Quat::default();
    let mut quat2 = Quat::default();
    if x {
        quat1 = Quat::from_rotation_y(std::f32::consts::PI);
    }
    if y {
        quat2 = Quat::from_rotation_x(std::f32::consts::PI);
    }
    quat1 * quat2
}
