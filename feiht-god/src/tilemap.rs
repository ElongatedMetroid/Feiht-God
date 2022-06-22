use std::{fs::File, io::{BufReader, BufRead}};

use bevy::prelude::*;

use crate::{sprites::{SpriteSheet, spawn_sprite}, TILE_SIZE};

pub struct TileMapPlugin;

#[derive(Component)]
pub struct TileCollider;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_simple_map);
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
                    Vec3::new(x as f32 * TILE_SIZE, -(y as f32) * TILE_SIZE, 100.0)
                );
                
                let mut curr_char = 'a' as usize;
                while curr_char < 'z' as usize {
                    if curr_char == char as usize {
                        commands.entity(tile).insert(TileCollider);
                    }

                    curr_char += 1;
                }

                tiles.push(tile);
            }
        }
    }

    commands.spawn()
        .insert(Name::new("Map"))
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .push_children(&tiles);
}