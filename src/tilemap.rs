use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use bevy::prelude::*;

use crate::{ascii::{spawn_ascii_sprite, AsciiSheet}, TILE_SIZE, RESOLUTION};
use bevy::input::ElementState;
use bevy_mouse_tracking_plugin::{MousePosWorld, MainCamera};

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
    let mut tiles = Vec::new();

    for x in 0..20 {
        for y in 0..100 {
            let tile = spawn_ascii_sprite(
                &mut commands,
                &ascii,
                12*16+4 as usize,
                Color::rgb(0.9, 0.9, 0.9),
                Vec3::new(x as f32 * TILE_SIZE, -(y as f32) * TILE_SIZE, 100.0),
            );
            tiles.push(tile);
        }
    }

    commands
        .spawn()
        .insert(Name::new("Map"))
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .push_children(&tiles);
}

fn spawn_wall(
    mut commands: Commands,
    event: Res<Input<MouseButton>>,
    ascii: Res<AsciiSheet>,
    mouse: Res<MousePosWorld>,
    window: Res<WindowDescriptor>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>
){
    if event.just_pressed(MouseButton::Right){
        let x = ((mouse.x / window.width) * 2.0 - 1.) * RESOLUTION;
        let y = (mouse.y / window.height) * 2.0 - 1.;
        let position = Vec3::new(x,y,0.0);

        let tile = spawn_ascii_sprite(
            &mut commands,
            &ascii,
            12*16+4 as usize,
            Color::rgb(0.9, 0.9, 0.9),
            position,
        );
        info!("Spawning tile at {}", position);
        commands.entity(tile);
    }
}
