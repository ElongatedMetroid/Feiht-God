use bevy::prelude::*;

use crate::{GameState, sprites::{SpriteSheet, spawn_sprite}, fadeout::create_fadeout};

pub struct CombatPlugin;

#[derive(Component)]
pub struct Enemy;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_update(GameState::Combat)
                .with_system(test_exit_combat)
                .with_system(combat_camera)
            )
            .add_system_set(SystemSet::on_enter(GameState::Combat).with_system(spawn_enemy))
            .add_system_set(SystemSet::on_exit(GameState::Combat).with_system(despawn_enemy));
    }
}

// center the camera to the enemy
// we will need the transform of the camera so we can edit the translation
fn combat_camera(mut camera_query: Query<&mut Transform, With<Camera>>) {
    let mut camera_transform = camera_query.single_mut();

    camera_transform.translation.x = 0.0;
    camera_transform.translation.y = 0.0;
}

fn spawn_enemy(
    // commands will be used to spawn a new entity
    mut commands: Commands, 
    // we will need the sprite_sheet to pass to spawn_sprite
    sprite_sheet: Res<SpriteSheet>
) {
    let sprite = spawn_sprite(
        &mut commands, 
        &sprite_sheet, 
        127, 
        Vec3::new(0.0, 0.5, 100.0),
        10.0
    );

    commands.entity(sprite)
        .insert(Enemy)
        .insert(Name::new("Face"));
}

fn despawn_enemy(
    // commands will be used to despawn the entity
    mut commands: Commands,
    // query for all the enemy entitys (so we can despawn them)
    enemy_query: Query<Entity, With<Enemy>>
)
{
    // iterate through all the enemys
    for entity in enemy_query.iter() {
        // and despawn the enemy and its children
        commands.entity(entity).despawn_recursive();
    }
}

fn test_exit_combat(
    mut commands: Commands,
    mut keyboard: ResMut<Input<KeyCode>>, 
    sprite_sheet: Res<SpriteSheet>
) {
    if keyboard.just_pressed(KeyCode::Space) {
        create_fadeout(&mut commands, GameState::Overworld, &sprite_sheet);
        keyboard.clear();
    }
}
