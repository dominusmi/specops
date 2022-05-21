use bevy::core::CorePlugin;
use bevy::input::InputPlugin;
use bevy::prelude::*;
use bevy::render::camera::{ScalingMode};
use bevy::window::{PresentMode, WindowPlugin};
use env_logger;
use bevy_mouse_tracking_plugin::MousePosPlugin;


use crate::ascii::AsciiPlugin;
use crate::debug::DebugPlugin;
use crate::player::PlayerPlugin;
use crate::tilemap::TileMapPlugin;
use crate::camera::CameraPlugin;
use bevy_rapier2d::plugin::{RapierPhysicsPlugin, NoUserData};
use bevy_rapier2d::prelude::RapierDebugRenderPlugin;


mod player;
mod debug;
mod ascii;
mod tilemap;
mod camera;

pub const RESOLUTION: f32 = 16.0 / 9.0;


fn main() {
    let height = 900.0;
    App::new()
        .insert_resource(WindowDescriptor {
            width: height * RESOLUTION,
            height,
            title: "Test".to_string(),
            resizable: false,
            present_mode: PresentMode::Immediate,
            .. Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(MousePosPlugin::SingleCamera)
        .add_plugin(CameraPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(AsciiPlugin)
        .add_plugin(TileMapPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .run();
}

pub const TILE_SIZE: f32 = 0.1;
