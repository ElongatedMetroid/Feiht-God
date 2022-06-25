use bevy::{prelude::*, sprite::collide_aabb::collide};
use bevy_inspector_egui::Inspectable;

use crate::{TILE_SIZE, sprites::{spawn_sprite, Facing}, sprites::{SpriteSheet, AnimationTimer}, tilemap::{TileCollider, EncounterSpawner}, GameState, fadeout::create_fadeout};

pub struct PlayerPlugin;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct EncounterTracker {
    timer: Timer
}

#[derive(Component, Inspectable)]
pub struct Player {
    pub has_moved: bool,
    speed: f32,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_player)
            .add_system_set(
                SystemSet::on_enter(GameState::Overworld).with_system(show_player))
            .add_system_set(
                SystemSet::on_exit(GameState::Overworld).with_system(hide_player))
            .add_system_set(SystemSet::on_update(GameState::Overworld)
                .with_system(player_movement.label("movement"))
                // execute camera system after player movement system
                .with_system(camera_follow.after("movement"))
                .with_system(animate_player_sprite)
                .with_system(player_encounter_checking.after("movement"))
            );
    }
}

fn hide_player(
    mut player_query: Query<&mut Visibility, With<Player>>,
    children_query: Query<&mut Children, With<Player>>,
    mut child_visibility_query: Query<&mut Visibility, Without<Player>>
) {
    let mut player_visibility = player_query.single_mut();
    player_visibility.is_visible = false;

    // if there is children to the player
    if let Ok(children) = children_query.get_single() {
        // for each child
        for child in children.iter() {
            // get the specific child entitys Visibility component from the query,
            // (since we queryed for all entitys with the Visibility component)
            if let Ok(mut child_vis) = child_visibility_query.get_mut(*child) {
                child_vis.is_visible = false;
            }
        }
    }
}

fn show_player(
    mut player_query: Query<&mut Visibility, With<Player>>,
    children_query: Query<&mut Children, With<Player>>,
    mut child_visibility_query: Query<&mut Visibility, Without<Player>>
) {
    let mut player_visibility = player_query.single_mut();
    player_visibility.is_visible = true;

    // if there is children to the player
    if let Ok(children) = children_query.get_single() {
        // for each child
        for child in children.iter() {
            // get the specific child entitys Visibility component from the query,
            // (since we queryed for all entitys with the Visibility component)
            if let Ok(mut child_vis) = child_visibility_query.get_mut(*child) {
                child_vis.is_visible = true;
            }
        }
    }
}

fn player_encounter_checking(
    mut commands: Commands,
    mut player_query: Query<(&Player, &mut EncounterTracker, &Transform)>,
    encounter_query: Query<&Transform, (With<EncounterSpawner>, Without<Player>)>,
    mut state: ResMut<State<GameState>>,
    time: Res<Time>,
    sprite_sheet: Res<SpriteSheet>
) {
    let (player, mut encounter_tracker, player_translation) = player_query.single_mut();
    let player_translation = player_translation.translation;

    // iterate through all the EncounterSpawner transforms returned from the query and if there is collision and the player has moved ...
    if encounter_query
        .iter()
        .any(|&transform| wall_collision_check(player_translation, transform.translation)) 
        && player.has_moved
    {
        // tick the timer every time the player is walking through an EncounterSpawner
        encounter_tracker.timer.tick(time.delta());

        // every time the timer finishes switch to combat state
        if encounter_tracker.timer.just_finished() {
            create_fadeout(&mut commands, GameState::Combat, &sprite_sheet);
        }
    }
}

fn camera_follow(
    // query for entitys with Transform and Player component this is not in a tuple because 
    // the Player component data does not need to be acessed, we just need to query for an 
    // entity that has the transform component (The data we will acess) and the player component,
    // (data we do not need to access)
    player_query: Query<&Transform, With<Player>>,
    // query for entitys with a Transform and Camera component but skip entitys with a 
    // player component because the player component could have a camera component
    mut camera_query: Query<&mut Transform, (Without<Player>, With<Camera>)>
) {
    // get transforms from querys with single (since only one entity matches the querys)
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}

fn player_movement(
    // query for the player, the players transform will need to be adjusted so it is a mutable reference 
    mut player_query: Query<(&mut Player, &mut Transform, &mut Facing)>, 
    // query for walls with colliders, we will need the transform of the walls 
    // (again without player is required because the Player component could have a TileCollider component, meaning more than one result/entity)
    wall_query: Query<&Transform, (With<TileCollider>, Without<Player>)>,
    // we will also need a keyboard input here so we will get the Input resource of type KeyCode
    keyboard: Res<Input<KeyCode>>, 
    // we will use the Time resource to multiply by delta time
    time: Res<Time>,
) {
    // get the transform and player component out of the query
    let (mut player, mut transform, mut facing) = player_query.single_mut();

    // (We can now check for keyboard input and edit the transform since we have a mutable reference to it)

    // add/subtract any movement from keypresses on the y axis
    let mut y_delta = 0.0;
    if keyboard.pressed(KeyCode::W) {
        *facing = Facing::UP;
        y_delta += player.speed * TILE_SIZE * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::S) {
        *facing = Facing::DOWN;
        y_delta -= player.speed * TILE_SIZE * time.delta_seconds();
    }

    // add/subtract any movement from keypresses on the x axis
    let mut x_delta = 0.0;
    if keyboard.pressed(KeyCode::D) {
        *facing = Facing::RIGHT;
        x_delta += player.speed * TILE_SIZE * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::A) {
        *facing = Facing::LEFT;
        x_delta -= player.speed * TILE_SIZE * time.delta_seconds();
    }

    // diagnal
    if y_delta != 0.0 && x_delta != 0.0 {
        y_delta /= 1.5;
        x_delta /= 1.5;
        if y_delta > 0.0 && x_delta > 0.0 {
            *facing = Facing::UP_RIGHT;
        } 
        else if y_delta > 0.0 && x_delta < 0.0 {
            *facing = Facing::UP_LEFT;
        } 
        else if y_delta < 0.0 && x_delta > 0.0 {
            *facing = Facing::DOWN_RIGHT;
        } 
        else if y_delta < 0.0 && x_delta < 0.0 {
            *facing = Facing::DOWN_LEFT;
        }
    }

    // the target as in where the player should be from the pressed buttons
    let target = transform.translation + Vec3::new(x_delta, 0.0, 0.0);
    // iterate through all the walls returned from the wall query and if there is no collision ...
    if !wall_query
        .iter()
        .any(|&transform| wall_collision_check(target, transform.translation)) 
    {
        // update the players position to what they pressed
        transform.translation = target;
    }

    // the target as in where the player should be from the pressed buttons
    let target = transform.translation + Vec3::new(0.0, y_delta, 0.0);
    // iterate through all the walls returned from the wall query and if there is no collision ...
    if !wall_query
        .iter()
        .any(|&transform| wall_collision_check(target, transform.translation)) 
    {
        // update the players position to what they pressed
        transform.translation = target;
    }

    if y_delta != 0.0 || x_delta != 0.0 {
        player.has_moved = true;
    } else {
        player.has_moved = false;
    }
}

fn wall_collision_check(
    // where the player should be from the key presses
    target_player_pos: Vec3,
    // query for all transforms with a TileCollider component
    wall_translation: Vec3
    // true -> collided, false -> no collision
) -> bool {
    let collision = collide(
        // center position of player collision rectangle
        target_player_pos,
        // dimensions of player collision rectangle
        Vec2::splat(TILE_SIZE * 0.9),
        // center postion of wall collision rectangle
        wall_translation,
        // dimensions of wall collision rectangle
        Vec2::splat(TILE_SIZE)
    );
    // if there is collision of any value return true (as in collision occured)
    collision.is_some()
}

// TODO: Add the other diagnal animations, create idle animations, and diagnal animation
fn animate_player_sprite(
    mut query: Query<(&mut TextureAtlasSprite, &mut Facing, &mut AnimationTimer, &Player)>,
    time: Res<Time>
) {
    let (mut sprite, direction, mut animation_timer,player) = query.get_single_mut().unwrap();
    animation_timer.0.tick(time.delta());

    // if the player has moved change the sprite to the movement in the particular direction
    if player.has_moved {   
        // every half a half a second (or every half of the animation timer duration),
        // switch the sprite that is being displayed
        if animation_timer.0.elapsed_secs() > animation_timer.0.duration().as_secs_f32() / 2.0{
            sprite.index = match *direction {
                Facing::DOWN_RIGHT => 15,
                Facing::DOWN_LEFT => 13,
                Facing::UP_RIGHT => 11,
                Facing::UP_LEFT => 9,

                Facing::DOWN => 7,
                Facing::UP => 5,
                Facing::LEFT => 3,
                Facing::RIGHT => 1,
            }
        } else {
            sprite.index = match *direction {
                Facing::DOWN_RIGHT => 16,
                Facing::DOWN_LEFT => 14,
                Facing::UP_RIGHT => 12,
                Facing::UP_LEFT => 10,

                Facing::DOWN => 8,
                Facing::UP => 6,
                Facing::LEFT => 4,
                Facing::RIGHT => 2,
            }
        }
    } else { // if the player stops moving go to "idle" position
        sprite.index = match *direction {
            Facing::DOWN_RIGHT => 15,
            Facing::DOWN_LEFT => 13,
            Facing::UP_RIGHT => 11,
            Facing::UP_LEFT => 9,

            Facing::DOWN => 7,
            Facing::UP => 5,
            Facing::LEFT => 3,
            Facing::RIGHT => 1,
        }
    }
}

fn spawn_player(
    // we need commands because we will be spawning a entity (inside of spawn_sprite),
    // we will also need commands because we will be adding components to our player entity
    mut commands: Commands, 
    // we will need the sprite sheet resource because we will be loading sprites
    sprites: Res<SpriteSheet>
) {
    // create a new player entity
    let player = spawn_sprite(
        &mut commands, 
        &sprites,
        // index 1 contains forward facing player sprite
        1,
        Vec3::new(5.0 * TILE_SIZE, -5.0 * TILE_SIZE, 900.0)
    );

    commands.entity(player)
        // add components to player entity
        .insert(Name::new("Player"))
        .insert(Player { 
            speed: 4.0, 
            has_moved: false 
        })
        .insert(Facing::RIGHT)
        .insert(AnimationTimer(Timer::from_seconds(0.5, true)))
        .insert(EncounterTracker {
            timer: Timer::from_seconds(1.0, true)
        });

}