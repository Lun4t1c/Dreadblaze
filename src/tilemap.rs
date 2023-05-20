use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use bevy::prelude::*;

use crate::{
    player::Player,
    GameState, TILE_SIZE, npc::Npc, graphics::{spawn_ground_tile_sprite, GroundTilesSheet, CharacterSheet, spawn_character_sprite, spawn_world_object_sprite, WorldObjectsSheet},
};

pub struct TileMapPlugin;

#[derive(Component)]
pub struct EncounterSpawner;

#[derive(Component)]
struct Map;

#[derive(Component)]
pub struct TileCollider;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(GameState::Overworld).with_system(create_simple_map))
            .add_system_set(SystemSet::on_resume(GameState::Overworld).with_system(show_map))
            .add_system_set(SystemSet::on_pause(GameState::Overworld).with_system(hide_map));
    }
}

fn create_simple_map(
    mut commands: Commands,
    ground_tiles: Res<GroundTilesSheet>,
    characters: Res<CharacterSheet>,
    world_objects: Res<WorldObjectsSheet>
) {
    let file = File::open("assets/map.txt").expect("No map file found");
    let mut tiles = Vec::new();

    for (y, line) in BufReader::new(file).lines().enumerate() {
        if let Ok(line) = line {
            for (x, char) in line.chars().enumerate() {
                let index = match char {
                    '.' => ground_tiles.sand,
                    '~' => ground_tiles.grass,
                    '#' => ground_tiles.wall,
                    _ => ground_tiles.sand
                };

                let tile = spawn_ground_tile_sprite(
                    &mut commands,
                    &ground_tiles,
                    index,
                    Vec3::new(x as f32 * TILE_SIZE, -(y as f32) * TILE_SIZE, 100.0),
                    Vec3::splat(1.0),
                );

                if char == '#' {
                    commands.entity(tile).insert(TileCollider);
                }
                if char == '~' {
                    let world_object_sprite = spawn_world_object_sprite(
                        &mut commands,
                        &world_objects,
                        world_objects.grass,
                        Vec3::new(0.0, 0.0, 150.0),
                        Vec3::splat(0.8),
                    );
                    commands.entity(tile)
                        .insert(Name::new("grass_tile".to_string()))
                        .insert(EncounterSpawner)
                        .add_child(world_object_sprite);
                        // .insert(FrameAnimation {
                        //     timer: Timer::from_seconds(0.2, true),
                        //     frames: world_objects.grass.to_vec(),
                        //     current_frame: 0
                        // });
                }
                if char == '@' {
                    let character_tile = spawn_character_sprite(
                        &mut commands,
                        &characters,
                        characters.healer,
                        Vec3::new(x as f32 * TILE_SIZE, -(y as f32) * TILE_SIZE, 150.0),
                        Vec3::splat(1.0)
                    );
                    tiles.push(character_tile);
                    commands.entity(tile).insert(Npc::Healer).insert(TileCollider);
                }

                tiles.push(tile);
            }
        }
    }

    commands
        .spawn()
        .insert(Map)
        .insert(Name::new("Map"))
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .push_children(&tiles);
}

fn hide_map(
    children_query: Query<&Children, With<Map>>,
    mut child_visibility_query: Query<&mut Visibility, Without<Map>>,
) {
    if let Ok(children) = children_query.get_single() {
        for child in children.iter() {
            if let Ok(mut child_vis) = child_visibility_query.get_mut(*child) {
                child_vis.is_visible = false;
            }
        }
    }
}

fn show_map(
    children_query: Query<&Children, With<Map>>,
    mut child_visibility_query: Query<&mut Visibility, Without<Player>>,
) {
    if let Ok(children) = children_query.get_single() {
        for child in children.iter() {
            if let Ok(mut child_vis) = child_visibility_query.get_mut(*child) {
                child_vis.is_visible = true;
            }
        }
    }
}
