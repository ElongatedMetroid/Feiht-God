use bevy::prelude::*;

use crate::{AsciiSheet, TILE_SIZE};

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player {
    speed: f32,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_player)
            .add_system(player_movement);
    }
}

// query for player, look for entitys that have Player component and transform component,
// we also want keyboard input in this system so we will get the Input resource of type KeyCode
fn player_movement(mut player_query: Query<(&Player, &mut Transform)>, keyboard: Res<Input<KeyCode>>, time: Res<Time>) {
    // get the transform out of the query
    let (player, mut transform) = player_query.single_mut();

    // (We can now check for keyboard input and edit the transform since we have a mutable reference to it)
    if keyboard.pressed(KeyCode::W) {
        transform.translation.y += 0.5 * player.speed * TILE_SIZE * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::A) {
        transform.translation.x -= 0.5 * player.speed * TILE_SIZE * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::S) {
        transform.translation.y -= 0.5 * player.speed * TILE_SIZE * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::D) {
        transform.translation.x += 0.5 * player.speed * TILE_SIZE * time.delta_seconds();
    }
}

fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>) {
    // create a texture atlas sprite, and set the index to 1 (the smile face on the sheet)
    let mut sprite = TextureAtlasSprite::new(1);
    sprite.color = Color::rgb(0.3, 0.3, 0.9);
    sprite.custom_size = Some(Vec2::splat(TILE_SIZE));

    let player = commands.spawn_bundle(SpriteSheetBundle {
        sprite,
        texture_atlas: ascii.0.clone(),
        transform: Transform { 
            translation: Vec3::new(0.0, 0.0, 900.0),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Name::new("Player"))
    .insert(Player {
        speed: 3.0,
    })
    .id();

    let mut background_sprite = TextureAtlasSprite::new(0);
    background_sprite.color = Color::rgb(0.5, 0.5, 0.5);
    background_sprite.custom_size = Some(Vec2::splat(TILE_SIZE));

    let background = commands.spawn_bundle(SpriteSheetBundle {
        sprite: background_sprite,
        texture_atlas: ascii.0.clone(),
        transform: Transform { 
            translation: Vec3::new(0.0, 0.0, -1.0),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Name::new("Background"))
    .id();

    // set the background to be a child of the player
    commands.entity(player).push_children(&[background]);
}