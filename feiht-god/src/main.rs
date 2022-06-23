#![allow(clippy::redundant_field_names)]

use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::window::*;

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 10.80 / 7.20;
pub const TILE_SIZE: f32 = 0.1;

mod player;
mod debug;
mod sprites;
mod tilemap;

use player::PlayerPlugin;
use debug::DebugPlugin;
use sprites::SpritePlugin;
use tilemap::TileMapPlugin;

fn main() {
    let height = 720.0;

    App::new()
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            width: height * RESOLUTION,
            height: height,
            title: String::from("Feiht God"),
            present_mode: PresentMode::Fifo,
            resizable: false,
            decorations: true,
            cursor_visible: true,
            cursor_locked: false,
            mode: WindowMode::Windowed,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(SpritePlugin)
        .add_plugin(TileMapPlugin)
        .add_startup_system(spawn_camera)
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