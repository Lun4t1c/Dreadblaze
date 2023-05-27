use::bevy::prelude::*;

use crate::{combat::EnemyType, TILE_SIZE};

pub struct GraphicsPlugin;

pub struct CharacterSheet {
    pub handle: Handle<TextureAtlas>,
    pub player_up: [usize; 3],
    pub player_down: [usize; 3],
    pub player_left: [usize; 3],
    pub player_right: [usize; 3],

    pub bat_frames: [usize; 3],
    pub ghost_frames: [usize; 3],

    pub healer: usize,
}

pub struct GroundTilesSheet {
    pub handle: Handle<TextureAtlas>,
    pub sand: usize,
    pub grass: usize,
    pub wall: usize,
}

pub struct WorldObjectsSheet {
    pub handle: Handle<TextureAtlas>,
    pub grass: [usize; 2],
}

pub struct VfxSheet {
    pub handle: Handle<TextureAtlas>,
    pub slash: usize,
    pub magic: usize,
}

pub enum FacingDirection {
    Up,
    Down,
    Left,
    Right
}

#[derive(Component)]
pub struct PlayerGraphics {
    pub facing: FacingDirection,
}

#[derive(Component)]
pub struct FrameAnimation {
    pub timer: Timer,
    pub frames: Vec<usize>,
    pub current_frame: usize
}

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(StartupStage::PreStartup, Self::load_graphics)
            .add_system(Self::frame_animation)
            .add_system(Self::update_player_graphics);
    }
}

pub fn spawn_enemy_sprite(
    commands: &mut Commands,
    characters: &CharacterSheet,
    translation: Vec3,
    enemy_type: EnemyType
) -> Entity {
    let mut sprite = TextureAtlasSprite::new(characters.bat_frames[0]);
    sprite.custom_size = Some(Vec2::splat(0.5));
    let animation = match enemy_type {
        EnemyType::Bat => FrameAnimation {
            timer: Timer::from_seconds(0.2, true),
            frames: characters.bat_frames.to_vec(),
            current_frame: 0,
        },
        EnemyType::Ghost => FrameAnimation {
            timer: Timer::from_seconds(0.2, true),
            frames: characters.ghost_frames.to_vec(),
            current_frame: 0,
        },
    };

    commands.spawn_bundle(SpriteSheetBundle {
        sprite: sprite,
        texture_atlas: characters.handle.clone(),
        transform: Transform {
            translation: translation,
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(animation)
    .id()
}

pub fn spawn_ground_tile_sprite(
    commands: &mut Commands,
    ground_tiles: &GroundTilesSheet,
    index: usize,
    translation: Vec3,
    scale: Vec3,
) -> Entity {
    let mut sprite = TextureAtlasSprite::new(index);
    sprite.custom_size = Some(Vec2::splat(TILE_SIZE));    

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: sprite,
            texture_atlas: ground_tiles.handle.clone(),
            transform: Transform {
                translation: translation,
                scale: scale,
                ..Default::default()
            },
            ..Default::default()
        })
        .id()
}

pub fn spawn_character_sprite(
    commands: &mut Commands,
    characters: &CharacterSheet,
    index: usize,
    translation: Vec3,
    scale: Vec3,
) -> Entity {
    let mut sprite = TextureAtlasSprite::new(index);
    sprite.custom_size = Some(Vec2::splat(TILE_SIZE));

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: sprite,
            texture_atlas: characters.handle.clone(),
            transform: Transform {
                translation: translation,
                scale: scale,
                ..Default::default()
            },
            ..Default::default()
        })
        .id()
}

pub fn spawn_world_object_sprite(
    commands: &mut Commands,
    world_objects_sheet: &WorldObjectsSheet,
    index: usize,
    translation: Vec3,
    scale: Vec3,
) -> Entity {
    let mut sprite = TextureAtlasSprite::new(index);
    sprite.custom_size = Some(Vec2::splat(TILE_SIZE));

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: sprite,
            texture_atlas: world_objects_sheet.handle.clone(),
            transform: Transform {
                translation: translation,
                scale: scale,
                ..Default::default()
            },
            ..Default::default()
        })
        .id()
}

impl GraphicsPlugin {
    fn load_graphics(
        mut commands: Commands,
        assets: Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>
    ) {
        // Characters sheet
        let characters_sheet_handle = assets.load("tilesets/characters_tileset.png");
        let characters_atlas = TextureAtlas::from_grid_with_padding(
            characters_sheet_handle, Vec2::splat(16.0), 12, 8, Vec2::splat(2.0)
        );
        let characters_atlas_handle = texture_atlases.add(characters_atlas);

        let characters_columns = 12;

        commands.insert_resource(CharacterSheet {
            handle: characters_atlas_handle,
            player_down: [characters_columns * 0 + 3, characters_columns * 0 + 4, characters_columns * 0 + 5],
            player_left: [characters_columns * 1 + 3, characters_columns * 1 + 4, characters_columns * 1 + 5],
            player_right: [characters_columns * 2 + 3, characters_columns * 2 + 4, characters_columns * 2 + 5],
            player_up: [characters_columns * 3 + 3, characters_columns * 3 + 4, characters_columns * 3 + 5],
            
            bat_frames: [characters_columns * 4 + 3, characters_columns * 4 + 4, characters_columns * 4 + 5],
            ghost_frames: [characters_columns * 4 + 6, characters_columns * 4 + 7, characters_columns * 4 + 8],

            healer: characters_columns * 0 + 6,
        });

        // Ground tiles sheet
        let ground_tiles_sheet_handle = assets.load("tilesets/ground_tileset.png");
        let ground_tiles_atlas = TextureAtlas::from_grid_with_padding(
            ground_tiles_sheet_handle, Vec2::splat(16.0), 32, 32, Vec2::splat(0.0)
        );
        let ground_tiles_atlas_handle = texture_atlases.add(ground_tiles_atlas);

        let ground_tiles_columns = 32;

        commands.insert_resource(GroundTilesSheet {
            handle: ground_tiles_atlas_handle,
            grass: ground_tiles_columns * 15 + 2,
            sand: ground_tiles_columns * 6 + 2,
            wall: ground_tiles_columns * 1 + 0,
        });

        // World objects sheet
        let world_objects_sheet_handle = assets.load("tilesets/pokemon_tileset.png");
        let world_objects_atlas = TextureAtlas::from_grid_with_padding(
            world_objects_sheet_handle, Vec2::splat(16.0), 120, 210, Vec2::splat(0.0)
        );
        let world_objects_atlas_handle = texture_atlases.add(world_objects_atlas);

        let world_objects_columns = 120;

        commands.insert_resource(WorldObjectsSheet {
            handle: world_objects_atlas_handle,
            grass: [world_objects_columns * 49 + 0, world_objects_columns * 53 + 0],
        });

        // VFX sheet
        let vfx_sheet_handle = assets.load("tilesets/Ascii.png");
        let vfx_atlas = TextureAtlas::from_grid_with_padding(
            vfx_sheet_handle, Vec2::splat(16.0), 16, 16, Vec2::splat(2.0)
        );
        let vfx_atlas_handle = texture_atlases.add(vfx_atlas);

        let vfx_columns = 16;

        commands.insert_resource(VfxSheet {
            handle: vfx_atlas_handle,
            slash: vfx_columns * 2 + 15,
            magic: vfx_columns * 4 + 15,
        });
    }

    fn update_player_graphics(
        mut sprites_query: Query<(&PlayerGraphics, &mut FrameAnimation), Changed<PlayerGraphics>>,
        characters: Res<CharacterSheet>
    ) {
        for (graphics, mut animation) in sprites_query.iter_mut() {
            animation.frames = match graphics.facing {
                FacingDirection::Up => characters.player_up.to_vec(),
                FacingDirection::Down => characters.player_down.to_vec(),
                FacingDirection::Left => characters.player_left.to_vec(),
                FacingDirection::Right => characters.player_right.to_vec(),
            }
        }
    }

    fn frame_animation(
        mut sprites_query: Query<(&mut TextureAtlasSprite, &mut FrameAnimation)>,
        time: Res<Time>
    ) {
        for (mut sprite, mut animation) in sprites_query.iter_mut() {
            animation.timer.tick(time.delta());
            if animation.timer.just_finished() {
                animation.current_frame = (animation.current_frame + 1) % animation.frames.len();
                sprite.index = animation.frames[animation.current_frame];
            }
        }
    }
}