use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::{TILE_SIZE};

pub struct SpritePlugin;

#[derive(Component, Inspectable)]
pub enum Facing {
    UP,
    UP_RIGHT,
    UP_LEFT,
    DOWN,
    DOWN_RIGHT,
    DOWN_LEFT,
    LEFT,
    RIGHT
}

pub struct AnimationTimer{
    pub timer: Timer,
    pub has_moved: bool
}

// create my own resource that holds a copy of the specific sprites sheet handle
pub struct SpriteSheet(Handle<TextureAtlas>);

impl Plugin for SpritePlugin {
    fn build(&self, app: &mut App) {
        app
            // we want this sprite sheet to be the first thing loaded so nothing trys
            // to acess a sprite while they are still not fully loaded
            .add_startup_system_to_stage(StartupStage::PreStartup, load_sprites)
            .insert_resource(AnimationTimer{ timer: Timer::from_seconds(0.5, true), has_moved: false });
    }
}

pub fn spawn_sprite(
    // to spawn the sprite
    commands: &mut Commands,
    // to get the single sprite, and needed to spawn a sprite bundle
    sprites: &SpriteSheet,
    // which sprite to draw from the sheet
    index: usize,
    // position of the entity
    translation: Vec3
) -> Entity {
        // create a new texture atlas sprite and set it to the index provided
        let mut sprite = TextureAtlasSprite::new(index);
        // set the size to the standard tile size
        sprite.custom_size = Some(Vec2::splat(TILE_SIZE));
    
        commands
            // spawn a new entity with the components contained 
            // in a bundle (a bundle is a collection of components)
            // We will use the SpriteSheetBundle than contains components for drawing
            // a single sprite from the texture atlas (sprite sheet)
            .spawn_bundle(SpriteSheetBundle {
                // single sprite from the texture atlas to be drawn
                sprite,
                // handle to the texture atlas that holds all sprites
                texture_atlas: sprites.0.clone(),
                // transformation that will be where the sprite is drawn
                transform: Transform { 
                    translation: translation,
                    ..Default::default()
                },
                ..Default::default()
            }
        ).id() // return id of the newly created [sprite] entity
}
 
fn load_sprites(
    // commands are needed here because we will be inserting a resource
    mut commands: Commands, 
    // we also need the asset server resource for loading the image (sprite sheet)
    assets: Res<AssetServer>,
    // we also need a mutable reference to the TextureAtlas asset manager 
    // because we will be adding an asset to the collection
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
) {
    let image = assets.load("Sprites.png");
    // create a texture atlas since it is a sprite sheet
    // the sprites sprite sheet is also padded so we will use from_grid_with_padding
    let atlas = TextureAtlas::from_grid_with_padding(
        image,
        Vec2::splat(15.0),
        32,
        32,
        Vec2::splat(1.0)
    );

    // create a handle to our new TextureAtlas (sprite sheet) named atlas_handle
    let atlas_handle = texture_atlases.add(atlas);
    // insert a resource that contains our handle, think of a handle as a pointer,
    // in this case its pointing to our atlas asset that we added to the collection
    // (This is for simplicity)
    commands.insert_resource(SpriteSheet(atlas_handle));
}