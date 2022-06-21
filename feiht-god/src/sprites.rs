use bevy::prelude::*;

use crate::TILE_SIZE;

pub struct SpritePlugin;

// create my own resource that holds a copy of the specific sprites sheet handle
pub struct SpriteSheet(Handle<TextureAtlas>);

impl Plugin for SpritePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(StartupStage::PreStartup, load_sprites);
    }
}

pub fn spawn_sprite(
    commands: &mut Commands,
    sprites: &SpriteSheet,
    index: usize,
    color: Color,
    translation: Vec3
) -> Entity {
        // create a texture atlas sprite, and set the index to 1 (the smile face on the sheet)
        let mut sprite = TextureAtlasSprite::new(index);
        sprite.color = color;
        sprite.custom_size = Some(Vec2::splat(TILE_SIZE));
    
        commands
            .spawn_bundle(SpriteSheetBundle {
                sprite,
                texture_atlas: sprites.0.clone(),
                transform: Transform { 
                translation: translation,
                ..Default::default()
            },
            ..Default::default()
        }).id()
}

// Needs commands because we will be adding a resource, 
// also needs to acess the asset server resource to load the image (from the default plugins)
// and needs mutable acess to the texture atlas asset manager
fn load_sprites(mut commands: Commands, assets: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    let image = assets.load("Sprites.png");
    // create a texture atlas since it is a sprite sheet
    // the sprites sprite sheet is also padded so we will use from_grid_with_padding
    let atlas = TextureAtlas::from_grid_with_padding(
        image,
        Vec2::splat(16.0),
        16,
        16,
        Vec2::splat(2.0)
    );

    // add atlas to texture atlases resource
    let atlas_handle = texture_atlases.add(atlas);
    commands.insert_resource(SpriteSheet(atlas_handle));
}