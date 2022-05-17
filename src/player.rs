use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use env_logger;
use log;

use std::f32::consts::FRAC_PI_2;

use crate::ascii::{AsciiSheet, spawn_ascii_sprite};
use crate::TILE_SIZE;
use bevy_mouse_tracking_plugin::MousePosWorld;

pub struct PlayerPlugin;

#[derive(Component, Inspectable)]
pub struct Player {
    speed: f32
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(player_movement)
            .add_system(player_direction)
            .add_system(camera_follow);
    }
}

fn player_direction(
    mut player_query: Query<(&Player, &mut Transform)>,
    mouse: Res<MousePosWorld>,
    time: Res<Time>,
    window: Res<WindowDescriptor>,
) {
    let (player, mut transform): (&Player, Mut<Transform>) = player_query.single_mut();

    let target = Vec2::new(mouse.x - window.width / 2., mouse.y - window.height / 2.).normalize();

    let cos = transform.rotation.w.powi(2) - transform.rotation.z.powi(2);
    let sin = 2. * transform.rotation.w * transform.rotation.z;

    let pos = Vec2::new(cos, sin);
    let angle = pos.angle_between(target ) - FRAC_PI_2;


    if angle > 0.1f32 {
        // for small angle, immediate movement (to avoid "damping" effect)
        let angle = angle * (FRAC_PI_2 / 0.25) * time.delta_seconds();
    }

    transform.rotate(Quat::from_axis_angle(Vec3::new(0.,0.,1.), angle));
}


fn player_movement(
    mut player_query: Query<(&Player, &mut Transform)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>
){
    let (player, mut transform): (&Player, Mut<Transform>) = player_query.single_mut();
    let move_dir = transform.rotation * Vec3::new(0., 1., 0.);

    if keyboard.pressed(KeyCode::W) {
        transform.translation += player.speed * time.delta_seconds() * TILE_SIZE * move_dir;
    }
    if keyboard.pressed(KeyCode::S){
        transform.translation -= player.speed * time.delta_seconds() * TILE_SIZE * move_dir;
    }
    if keyboard.pressed(KeyCode::A){
        let move_dir = Quat::from_axis_angle(Vec3::new(0.,0.,1.), FRAC_PI_2) * move_dir;
        transform.translation += player.speed * time.delta_seconds() * TILE_SIZE * move_dir;
    }
    if keyboard.pressed(KeyCode::D){
        let move_dir = Quat::from_axis_angle(Vec3::new(0.,0.,1.), FRAC_PI_2) * move_dir;
        transform.translation -= player.speed * time.delta_seconds() * TILE_SIZE * move_dir;
    }
    if keyboard.pressed(KeyCode::E){
        transform.rotate(Quat::from_axis_angle(Vec3::new(0.,0.,1.), 0.1))
    }
}

fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (Without<Player>, With<Camera>)>,
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}


pub fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>){
    let player = spawn_ascii_sprite(
        &mut commands, &ascii, 1, Color::rgb(0.3, 0.3, 0.9),
        Vec3::new(0.0,0.0,900.)
    );
    commands.entity(player)
        .insert(Name::new("Player"))
        .insert(Player{ speed: 3.0 })
        .id();

    let bg = spawn_ascii_sprite(
        &mut commands, &ascii, 0, Color::rgb(0.5, 0.5, 0.5),
        Vec3::new(0.,0.,900.0)
    );

    commands.entity(bg)
        .insert(Name::new("PlayerBackground"))
        .id();

    commands.entity(player).push_children(&[bg]);
}
