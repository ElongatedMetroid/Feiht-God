use bevy::{prelude::*, sprite::collide_aabb::collide};
use bevy_inspector_egui::Inspectable;

use crate::{TILE_SIZE, sprites::{spawn_sprite, Facing}, sprites::{SpriteSheet, AnimationTimer}, tilemap::TileCollider};

pub struct PlayerPlugin;

#[derive(Component, Inspectable)]
pub struct Player {
    pub has_moved: bool,
    speed: f32,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_player)
            .add_system(player_movement.label("movement"))
            // execute camera system after player movement system
            .add_system(camera_follow.after("movement"))
            .add_system(animate_player_sprite.after("movement"));
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
        if y_delta > 0.0 && x_delta > 0.0 {
            *facing = Facing::UP_RIGHT;
        } 
        if y_delta > 0.0 && x_delta < 0.0 {
            *facing = Facing::UP_LEFT;
        } 
        if y_delta < 0.0 && x_delta > 0.0 {
            *facing = Facing::DOWN_RIGHT;
        } 
        if y_delta < 0.0 && x_delta < 0.0 {
            *facing = Facing::DOWN_LEFT;
        }
    }

    // the target as in where the player should be from the pressed buttons
    let target = transform.translation + Vec3::new(x_delta, 0.0, 0.0);
    // if there is no collision between the player and a wall ... 
    if !wall_collision_check(target, &wall_query) {
        // update the players position to what they pressed
        transform.translation = target;
    }

    // the target as in where the player should be from the pressed buttons
    let target = transform.translation + Vec3::new(0.0, y_delta, 0.0);
    // if there is no collision between the player and a wall ...
    if !wall_collision_check(target, &wall_query) {
        // update the players position to what they pressed
        transform.translation = target;
    }

    if y_delta + x_delta == 0.0 {
        player.has_moved = false;
    } else {
        player.has_moved = true;
    }
}

fn wall_collision_check(
    // where the player should be from the key presses
    target_player_pos: Vec3,
    // query for all transforms with a TileCollider component
    wall_query: &Query<&Transform, (With<TileCollider>, Without<Player>)>
    // true -> collided, false -> no collision
) -> bool {
    // iterate through each wall transform
    for wall_transform in wall_query.iter() {
        // create a collision "box"
        let collision = collide(
            // center position of player collision rectangle
            target_player_pos,
            // dimensions of player collision rectangle
            Vec2::splat(TILE_SIZE * 0.9),

            // center postion of wall collision rectangle
            wall_transform.translation,
            // dimensions of wall collision rectangle
            Vec2::splat(TILE_SIZE)
        );
        // if there is collision of any value return true (as in collision occured)
        if collision.is_some() {
            return true;
        }
    }

    // no tiles returned collision of any intensity
    false
}

// TODO: Add the other diagnal animations, and create idle animations
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
                Facing::UP_RIGHT => 9,
                Facing::DOWN => 7,
                Facing::UP => 5,
                Facing::LEFT => 3,
                Facing::RIGHT => 1,
                _ => 0,
            }
        } else {
            sprite.index = match *direction {
                Facing::UP_RIGHT => 10,
                Facing::DOWN => 8,
                Facing::UP => 6,
                Facing::LEFT => 4,
                Facing::RIGHT => 2,
                _ => 0,
            }
        }
    } else { // if the player stops moving go to "idle" position
        sprite.index = match *direction {
            Facing::UP_RIGHT => 9,
            Facing::DOWN => 7,
            Facing::UP => 5,
            Facing::LEFT => 3,
            Facing::RIGHT => 1,
            _ => 0,
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
        Vec3::new(2.0 * TILE_SIZE, -2.0 * TILE_SIZE, 900.0)
    );

    commands.entity(player)
        // add components to player entity
        .insert(Name::new("Player"))
        .insert(Player { speed: 3.0, has_moved: false })
        .insert(Facing::RIGHT)
        .insert(AnimationTimer(Timer::from_seconds(0.5, true)));

    // let background = spawn_sprite(
    //     &mut commands, 
    //     &sprites, 
    //     0,  
    //     Vec3::new(0.0, 0.0, -1.0)
    // );

    // commands.entity(background)
    //     // add components to background entity
    //     .insert(Name::new("Background"));

    // // set the background to be a child of the player
    // commands.entity(player).push_children(&[background]);
}