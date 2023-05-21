use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use bevy::prelude::*;

use crate::{
    GameState, TILE_SIZE, npc::Npc, graphics::{spawn_ground_tile_sprite, GroundTilesSheet, CharacterSheet, spawn_character_sprite, spawn_world_object_sprite, WorldObjectsSheet, FrameAnimation},
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
            .add_system_set(SystemSet::on_resume(GameState::Overworld).with_system(show_map_recursive))
            .add_system_set(SystemSet::on_pause(GameState::Overworld).with_system(hide_map_recursive));
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
                        world_objects.grass[0],
                        Vec3::new(0.0, 0.0, 150.0),
                        Vec3::splat(0.8),
                    );
                    commands.entity(world_object_sprite)
                        .insert(FrameAnimation {
                            timer: Timer::from_seconds(0.5, true),
                            frames: world_objects.grass.to_vec(),
                            current_frame: 0
                    });
                    commands.entity(tile)
                        .insert(Name::new("grass_tile".to_string()))
                        .insert(EncounterSpawner)
                        .add_child(world_object_sprite);
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

fn hide_map_recursive(
    entities: Query<Entity, With<Map>>,
    mut visibility_query: Query<&mut Visibility>,
    children_query: Query<&Children>,
) {
    for entity in entities.iter() {
        hide_entity_recursive(entity, &mut visibility_query, &children_query);
    }
}

fn show_map_recursive(
    entities: Query<Entity, With<Map>>,
    mut visibility_query: Query<&mut Visibility>,
    children_query: Query<&Children>,
) {
    for entity in entities.iter() {
        show_entity_recursive(entity, &mut visibility_query, &children_query);
    }
}

fn hide_entity_recursive(
    entity: Entity,
    mut visibility_query: &mut Query<&mut Visibility>,
    children_query: &Query<&Children>,
) {
    // Hide the current entity
    if let Ok(mut visibility) = visibility_query.get_mut(entity) {
        visibility.is_visible = false;
    }

    // Retrieve the children entities
    if let Ok(children) = children_query.get(entity) {
        // Recursively hide the children's children
        for &child_entity in children.iter() {
            hide_entity_recursive(child_entity, &mut visibility_query, &children_query);
        }
    }
}

fn show_entity_recursive(
    entity: Entity,
    mut visibility_query: &mut Query<&mut Visibility>,
    children_query: &Query<&Children>,
) {
    // Show the current entity
    if let Ok(mut visibility) = visibility_query.get_mut(entity) {
        visibility.is_visible = true;
    }

    // Retrieve the children entities
    if let Ok(children) = children_query.get(entity) {
        // Recursively show the children's children
        for &child_entity in children.iter() {
            show_entity_recursive(child_entity, &mut visibility_query, &children_query);
        }
    }
}
