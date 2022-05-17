use bevy::prelude::*;
use bevy::core::CorePlugin;
use bevy::input::InputPlugin;
use bevy::window::WindowPlugin;
use bevy::render::camera::ScalingMode;


pub const RESOLUTION: f32 = 16.0 / 9.0;

struct AsciiSheet(Handle<TextureAtlas>);


pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        
    }

    fn name(&self) -> &str {
        todo!()
    }
}


fn load_ascii(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>){
    let image =  assets.load("Ascii.png");
    let atlas = TextureAtlas::from_grid_with_padding(
        image,
        Vec2::splat(9.0),
        16,
        16,
        Vec2::splat(2.0)
    );
    let atlas_handle = texture_atlases.add(atlas);
    commands.insert_resource(AsciiSheet(atlas_handle));
}

fn spawn_camera(mut commands: Commands){
    let mut camera = OrthographicCameraBundle::new_2d();

    camera.orthographic_projection.top = 1.0;
    camera.orthographic_projection.bottom = -1.0;
    camera.orthographic_projection.right = 1.0 * RESOLUTION;
    camera.orthographic_projection.left = - 1.0 * RESOLUTION;

    camera.orthographic_projection.scaling_mode = ScalingMode::None;

    commands.spawn_bundle(camera);
}

fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>){
    let mut sprite = TextureAtlasSprite::new(1);
    sprite.color = Color::rgb(0.3, 0.3, 0.9);
    sprite.custom_size = Some(Vec2::splat(1.0));

    let player = commands.spawn_bundle(SpriteSheetBundle{
            sprite: sprite,
            texture_atlas: ascii.0.clone(),
            transform: Transform {
                translation: Vec3::new(0.,0.,900.0),
                .. Default::default()
            },
            .. Default::default()
        }
    ).insert(Name::new("Player")).id();

    let mut bg_sprite = TextureAtlasSprite::new(0);
    bg_sprite.color = Color::rgb(0.5, 0.5, 0.5);
    bg_sprite.custom_size = Some(Vec2::splat(1.0));

    let background = commands.spawn_bundle(
        SpriteSheetBundle{
            sprite: bg_sprite,
            texture_atlas: ascii.0.clone(),
            transform: Transform {
                translation: Vec3::new(0.,0.,900.0),
                .. Default::default()
            },
            .. Default::default()
        }
    ).insert(Name::new("Background")).id();

    commands.entity(player).add_child(background);
}

fn main() {
    let height = 900.0;
    App::new()
        .insert_resource(WindowDescriptor {
            width: height * RESOLUTION,
            height: height,
            title: "Test".to_string(),
            resizable: false,
            .. Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_player)
        .add_startup_system_to_stage(StartupStage::PreStartup, load_ascii)
        .run();
}