// use std::fs::File;
// use std::io::{BufRead, BufReader};
// use bevy::prelude::*;
//
// use crate::ascii::{AsciiSheet, spawn_ascii_sprite};
// use crate::TILE_SCALE;
//
// pub struct TileMapPlugin;
//
// // #[derive(Component)]
// // pub struct TileCollider;
//
// impl Plugin for TileMapPlugin {
//     fn build(&self, app: &mut App) {
//         app.add_startup_system(create_simple_map);
//     }
// }
//
// fn create_simple_map(mut commands: Commands, ascii: Res<AsciiSheet>) {
//     let file = File::open("assets/map.txt").expect("no map file found");
//     let mut tiles = Vec::new();
//
//     for (y, line) in BufReader::new(file).lines().enumerate() {
//         if let Ok(line) = line {
//             for (x, char) in line.chars().enumerate() {
//                 let tile = spawn_ascii_sprite(
//                     &mut commands,
//                     &ascii,
//                     char as usize,
//                     Color::WHITE,
//                     Vec3::new(x as f32 * TILE_SCALE, -(y as f32) * TILE_SCALE, 100.0),
//                 );
//                 if char == '#' {
//                     commands.entity(tile).insert(TileCollider);
//                 }
//                 tiles.push(tile);
//             }
//         }
//     }
//
//     commands.spawn()
//         .insert(Name::new("Map"))
//         .insert(Transform::default())
//         .insert(GlobalTransform::default())
//         .push_children(&tiles);
// }