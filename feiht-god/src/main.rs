#![allow(clippy::redundant_field_names)]

use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::window::*;

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 10.80 / 7.20;
pub const TILE_SIZE: f32 = 0.1;

mod player;
mod debug;

use player::PlayerPlugin;
use debug::DebugPlugin;

fn main() {
    let height = 720.0;

    App::new()
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            width: height * RESOLUTION,
            height: height,
            position: Some(Vec2::splat(100.0)),
            resize_constraints: WindowResizeConstraints {
                min_width: height * RESOLUTION,
                min_height: height,
                max_width: 1920.0,
                max_height: 1080.0,
            },
            scale_factor_override: None,
            title: String::from("Feiht God"),
            present_mode: PresentMode::Fifo,
            resizable: true,
            decorations: true,
            cursor_visible: true,
            cursor_locked: false,
            mode: WindowMode::Windowed,
            transparent: false
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(DebugPlugin)
        .add_startup_system(spawn_camera)
        .add_startup_system_to_stage(StartupStage::PreStartup, load_ascii)
        .run();
}

// camera system
fn spawn_camera(mut commands: Commands) {
    // Create an orthographic camera bundle, a bundle is a group of components packaged for easy use
    let mut camera = OrthographicCameraBundle::new_2d();

    // setup normalized cordinate system
    camera.orthographic_projection.top = 1.0;
    camera.orthographic_projection.bottom = -1.0;
    camera.orthographic_projection.right = 1.0 * RESOLUTION;
    camera.orthographic_projection.left = -1.0 * RESOLUTION;

    // Simple pixel art
    camera.orthographic_projection.scaling_mode = ScalingMode::None;

    // Create a new entity with all the components in the bundle
    commands.spawn_bundle(camera);
}

// create my own resource that holds a copy of the specific ascii sheet handle
struct AsciiSheet(Handle<TextureAtlas>);

// Needs commands because we will be adding a resource, 
// also needs to acess the asset server resource to load the image (from the default plugins)
// and needs mutable acess to the texture atlas asset manager
fn load_ascii(mut commands: Commands, assets: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    let image = assets.load("Ascii.png");
    // create a texture atlas since it is a sprite sheet
    // the ascii sprite sheet is also padded so we will use from_grid_with_padding
    let atlas = TextureAtlas::from_grid_with_padding(
        image,
        Vec2::splat(9.0),
        16,
        16,
        Vec2::splat(2.0)
    );

    // add atlas to texture atlases resource
    let atlas_handle = texture_atlases.add(atlas);
    commands.insert_resource(AsciiSheet(atlas_handle));
}