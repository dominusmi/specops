use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use env_logger;
use log;

use std::f32::consts::FRAC_PI_2;

use crate::ascii::{AsciiSheet, spawn_ascii_sprite};
use crate::TILE_SIZE;
use bevy_mouse_tracking_plugin::MousePosWorld;
use bevy::input::mouse::MouseButtonInput;
use bevy::input::ElementState;
use bevy_rapier2d::prelude::*;

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

fn forward(transform: &Transform) -> Vec3 {
    return transform.rotation * Vec3::new(0., 1., 0.)
}

fn player_movement(
    mut player_query: Query<(&Player, &mut Transform, &mut Velocity)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>
){
    let (player, mut transform, mut rb_vels): (&Player, Mut<Transform>, Mut<Velocity>) = player_query.single_mut();

    let mut move_dir = transform.rotation * Vec3::new(0., 0., 0.);
    let mut move_requested = false;

    if keyboard.pressed(KeyCode::W){
        move_dir = forward(&transform);
        move_requested = true;
    }
    else if keyboard.pressed(KeyCode::S){
        move_dir = forward(&transform) * -1.;
        move_requested = true;

    }
    if keyboard.pressed(KeyCode::A){
        move_dir = Quat::from_axis_angle(Vec3::new(0.,0.,1.), FRAC_PI_2) * forward(&transform) + move_dir;
        move_dir = move_dir.normalize();
        move_requested = true;
    }

    else if keyboard.pressed(KeyCode::D){
        move_dir = Quat::from_axis_angle(Vec3::new(0.,0.,1.), FRAC_PI_2) * forward(&transform) * -1. + move_dir;
        move_dir = move_dir.normalize();
        move_requested = true;
    }
    if move_requested {
        rb_vels.linvel = move_dir.truncate() * player.speed;
        // transform.translation += player.speed * time.delta_seconds() * TILE_SIZE * move_dir;
    }
    else {
        rb_vels.linvel = Vect::splat(0.0);
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


pub fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>, mut rapier_config: ResMut<RapierConfiguration>){
    rapier_config.gravity = Vec2::ZERO;

    let player = spawn_ascii_sprite(
        &mut commands, &ascii, 1, Color::rgb(0.3, 0.3, 0.9),
        Some(Vec2::splat(TILE_SIZE)),
        Vec3::new(0.0,0.0,900.),
    );


    commands.entity(player)
        .insert(Name::new("Player"))
        .insert(Player{ speed: 1.0 })
        .insert(RigidBody::Dynamic)
        .insert(Velocity::zero())
        .insert(Collider::cuboid(TILE_SIZE/2.0, TILE_SIZE/2.0))
        .id();
}
