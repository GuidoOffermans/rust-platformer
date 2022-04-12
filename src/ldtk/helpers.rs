use bevy::{prelude::*};

// LDtk provides pixel locations starting in the top left. For Bevy we need to
// flip the Y axis and offset from the center of the screen.
pub fn convert_to_world_coordinates(
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
pub fn flip(x: bool, y: bool) -> Quat {
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