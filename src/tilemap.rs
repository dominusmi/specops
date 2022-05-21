use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use bevy::prelude::*;

use crate::{ascii::{spawn_ascii_sprite, AsciiSheet}, TILE_SIZE, RESOLUTION};
use bevy::input::ElementState;
use bevy_mouse_tracking_plugin::{MousePosWorld, MainCamera};
use bevy::utils::tracing::Instrument;
use bevy_rapier2d::prelude::RigidBody;
use bevy_rapier2d::geometry::Collider;
use crate::player::Player;

pub struct TileMapPlugin;

#[derive(Component)]
pub struct TileCollider;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_simple_map)
            .add_system(spawn_wall);
    }
}

fn create_simple_map(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let tile = spawn_ascii_sprite(
        &mut commands,
        &ascii,
        12*16+4 as usize,
        Color::rgb(0.9, 0.9, 0.9),
        Some(Vec2::splat(TILE_SIZE)),
        Vec3::new(0.0, -0.3, 0.0),
    );
    commands.entity(tile)
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(TILE_SIZE/2.0, TILE_SIZE/8.0));

    // let mut tiles = Vec::new();
    // for x in 0..20 {
    //     for y in 4..100 {
    //         let tile = spawn_ascii_sprite(
    //             &mut commands,
    //             &ascii,
    //             12*16+4 as usize,
    //             Color::rgb(0.9, 0.9, 0.9),
    //             Some(Vec2::splat(TILE_SIZE)),
    //             Vec3::new(x as f32 * TILE_SIZE, -(y as f32) * TILE_SIZE, 100.0),
    //         );
    //         tiles.push(tile);
    //     }
    // }
    //
    // commands
    //     .spawn()
    //     .insert(Name::new("Map"))
    //     .insert(RigidBody::Fixed)
    //     .insert(Collider::cuboid(TILE_SIZE/2.0, TILE_SIZE/8.0))
    //     .push_children(&tiles);
}

fn spawn_wall(
    mut commands: Commands,
    event: Res<Input<MouseButton>>,
    ascii: Res<AsciiSheet>,
    mouse: Res<MousePosWorld>,
    window: Res<WindowDescriptor>,
    player_query: Query<(&Player, &Transform)>
){
    let (player, transform): (&Player, &Transform) = player_query.single();
    if event.just_pressed(MouseButton::Right){
        let x = ((mouse.x / window.width) * 2.0 - 1.) * RESOLUTION + transform.translation.x;
        let y = ((mouse.y / window.height) * 2.0 - 1.) + transform.translation.y;
        let position = Vec3::new(x,y,0.0);

        let tile = spawn_ascii_sprite(
            &mut commands,
            &ascii,
            12*16+4 as usize,
            Color::rgb(0.9, 0.9, 0.9),
            Some(Vec2::splat(TILE_SIZE)),
            position,
        );
        info!("Spawning tile at {}", position);
        commands.entity(tile)
            .insert(RigidBody::Fixed)
            .insert(Collider::cuboid(TILE_SIZE/2.0, TILE_SIZE/8.0));
    }
}
