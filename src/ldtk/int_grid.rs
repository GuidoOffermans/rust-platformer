// fn display_color(
//     layer_info: LayerInfo,
//     tile: &i64,
//     commands: &mut Commands,
//     handle: Handle<ColorMaterial>,
// ) {
//     let x = *tile as i32 % layer_info.grid_width;
//     let y = *tile as i32 / layer_info.grid_width;
//     commands.spawn().insert_bundle(SpriteBundle {
//         // material: handle,
//         sprite: Sprite {
//             custom_size: Option::from(Vec2::new(layer_info.grid_cell_size as f32, layer_info.grid_cell_size as f32)),
//             ..Default::default()
//         },
//         transform: Transform {
//             translation: convert_to_world(
//                 layer_info.px_width,
//                 layer_info.px_height,
//                 layer_info.grid_cell_size,
//                 TILE_SCALE,
//                 x * layer_info.grid_cell_size,
//                 y as i32 * layer_info.grid_cell_size,
//                 layer_info.z_index,
//             ),
//             scale: Vec3::splat(TILE_SCALE),
//             ..Default::default()
//         },
//         ..Default::default()
//     });
// }