use std::{fs::File, io::{BufReader, BufRead}};

use bevy::prelude::*;

use crate::{sprites::{SpriteSheet, spawn_sprite}, TILE_SIZE, GameState};

pub struct TileMapPlugin;

#[derive(Component)]
struct Map;

#[derive(Component)]
pub struct EncounterSpawner;

#[derive(Component)]
pub struct TileCollider;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(create_simple_map)
            .add_system_set(
                SystemSet::on_enter(GameState::Overworld).with_system(show_map))
            .add_system_set(
                SystemSet::on_exit(GameState::Overworld).with_system(hide_map))
            
            ;
    }
}

fn hide_map(
    // query for the children of the Map, this will be used to check if 
    // the map has any children
    children_query: Query<&mut Children, With<Map>>,
    // query for the visibility component, this will be used to toggle
    // off the visibility of the maps children (if the map has any)
    mut child_visibility_query: Query<&mut Visibility, Without<Map>>
) {
    // if there is children to the map
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

fn show_map(
    // query for the children of the Map, this will be used to check if 
    // the map has any children
    children_query: Query<&mut Children, With<Map>>,
    // query for the visibility component, this will be used to toggle
    // on the visibility of the maps children (if the map has any)
    mut child_visibility_query: Query<&mut Visibility, Without<Map>>
) {
    // if there is children to the map
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

fn create_simple_map(mut commands: Commands, sprites: Res<SpriteSheet>) {
    let file = File::open("assets/map.txt")
        .expect("Map is missing!");
    let mut tiles = Vec::new();

    for (y, line) in BufReader::new(file).lines().enumerate() {
        if let Ok(line) = line {
            for (x, char) in line.chars().enumerate() {
                let tile = spawn_sprite(
                    &mut commands, 
                    &sprites, 
                    char as usize, 
                    Vec3::new(x as f32 * TILE_SIZE, -(y as f32) * TILE_SIZE, 100.0),
                    1.0
                );
                
                let mut curr_char = 'a' as usize;
                while curr_char < 'z' as usize {
                    if curr_char == char as usize {
                        commands.entity(tile).insert(TileCollider);
                    }

                    curr_char += 1;
                }
                if char == '!' {
                    commands.entity(tile).insert(EncounterSpawner);
                }

                tiles.push(tile);
            }
        }
    }
    commands.spawn()
        .insert(Map)
        .insert(Name::new("Map"))
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .push_children(&tiles);
}