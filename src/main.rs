#![allow(clippy::redundant_field_names)]
use bevy::{prelude::*, render::camera::ScalingMode};

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const TILE_SIZE: f32 = 0.1;

mod player;
mod debug;
mod ascii;

use player::PlayerPlugin;
use debug::DebugPlugin;
use ascii::AsciiPlugin;

fn main() {
    App::new()
    .insert_resource(ClearColor(CLEAR))
    .insert_resource(WindowDescriptor {
        width: 1600.0,
        height: 900.0,
        title: "Dreadblaze".to_string(),
        vsync: true,
        resizable: false,
        ..Default::default()
    })
    .add_startup_system(spawn_camera)    
    .add_plugins(DefaultPlugins)
    .add_plugin(PlayerPlugin)
    .add_plugin(AsciiPlugin)
    .add_plugin(DebugPlugin)
    .run();
}

fn spawn_camera(mut commands: Commands) {
    let mut camera: OrthographicCameraBundle = OrthographicCameraBundle::new_2d();

    camera.orthographic_projection.top = 1.0;
    camera.orthographic_projection.bottom = -1.0;

    camera.orthographic_projection.right = 1.0 * RESOLUTION;
    camera.orthographic_projection.left = -1.0 * RESOLUTION;

    camera.orthographic_projection.scaling_mode = ScalingMode::None;

    commands.spawn_bundle(camera);
}