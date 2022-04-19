use bevy::{
    prelude::*,
    sprite::collide_aabb::collide,
};
use bevy::render::camera::Camera2d;
use bevy_inspector_egui::Inspectable;

use crate::{AppState, TILE_SCALE, TileCollider};
use crate::game::{
    ascii::{AsciiSheet, spawn_ascii_sprite},
};

pub const JUMP_HEIGHT: f32 = 10.0 * 16.0 * TILE_SCALE as f32;
pub const JUMP_TIME_TO_PEAK: f32 = 0.5;
pub const JUMP_TIME_TO_DESCENT: f32 = 0.3;

pub const JUMP_VELOCITY: f32 = ((2.0 * JUMP_HEIGHT) / JUMP_TIME_TO_PEAK) * -1.0;
pub const JUMP_GRAVITY: f32 = ((-2.0 * JUMP_HEIGHT) / (JUMP_TIME_TO_PEAK * JUMP_TIME_TO_PEAK)) * -1.0;
pub const FALL_GRAVITY: f32 = ((-2.0 * JUMP_HEIGHT) / (JUMP_TIME_TO_DESCENT * JUMP_TIME_TO_DESCENT)) * -1.0;

#[derive(Component, Inspectable)]
pub struct Player {
    speed: f32,
    grounded: bool,
    y_velocity: f32,
    jump_speed: f32,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(AppState::InGame)
                .with_system(spawn_player))
            .add_system_set(SystemSet::on_update(AppState::InGame)
                .with_system(player_movement)
                .with_system(follow_camera)
            );
    }
}

fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let player = spawn_ascii_sprite(
        &mut commands,
        &ascii,
        1,
        Color::WHITE,
        Vec3::new(0.0, 200.0, 50.0),
    );
    commands
        .entity(player)
        .insert(Name::new("Player"))
        .insert(Player { speed: 300.0, grounded: false, y_velocity: -5.0, jump_speed: 300.0 })
        .id();
}

fn follow_camera(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (Without<Player>, With<Camera2d>)>,
) {
    let player_transform = player_query.single();
    let mut camera_tranform = camera_query.single_mut();

    camera_tranform.translation.x = player_transform.translation.x;
    camera_tranform.translation.y = player_transform.translation.y;
}

fn player_movement(
    mut player_query: Query<(&mut Player, &mut Transform)>,
    wall_query: Query<&Transform, (With<TileCollider>, Without<Player>)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut player, mut transform) = player_query.single_mut();
    // println!("{:?}", &JUMP_VELOCITY);
    if player.grounded && keyboard.just_pressed(KeyCode::W) {
        player.y_velocity += -JUMP_VELOCITY;
    }

    player.y_velocity -= get_gravity(player.y_velocity) * time.delta_seconds();

    let y_delta = player.y_velocity * time.delta_seconds();

    let mut x_delta = 0.0;
    if keyboard.pressed(KeyCode::A) {
        x_delta -= player.speed * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::D) {
        x_delta += player.speed * time.delta_seconds();
    }

    let target = transform.translation + Vec3::new(x_delta, 0.0, 0.0);
    if wall_collision_check(target, &wall_query) {
        transform.translation = target;
    }

    let target = transform.translation + Vec3::new(0.0, y_delta, 0.0);
    if wall_collision_check(target, &wall_query) {
        transform.translation = target;
        player.grounded = false;
    } else {
        player.y_velocity = 0.0;
        player.grounded = true;
    }
}

fn get_gravity(y_velocity: f32) -> f32 {
    if y_velocity < 0.0 {
        return JUMP_GRAVITY;
    }
    return FALL_GRAVITY;
}

fn wall_collision_check(
    target_player_pos: Vec3,
    wall_query: &Query<&Transform, (With<TileCollider>, Without<Player>)>,
) -> bool {
    for wall_transform in wall_query.iter() {
        let collision = collide(
            target_player_pos,
            Vec2::new(50.0 * TILE_SCALE, 37.0 * TILE_SCALE),
            wall_transform.translation,
            Vec2::splat(16.0 * TILE_SCALE),
        );
        if collision.is_some() {
            return false;
        }
    }
    true
}


