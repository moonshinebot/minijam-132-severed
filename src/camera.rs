use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::render::view::RenderLayers;

const REFERENCE_SCREEN_WIDTH: f32 = 213.0;
const REFERENCE_SCREEN_HEIGHT: f32 = 120.0;

/// Plugin adding primary camera handling for the game.
pub struct MainCameraPlugin;

/// Marker for the main game camera.
#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct UiCamera;

impl Plugin for MainCameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup);
    }
}

/// Spawns a main camera into the world.
fn setup(mut commands: Commands) {
    commands.spawn((
        MainCamera {},
        Camera2dBundle {
            projection: OrthographicProjection {
                scaling_mode: ScalingMode::AutoMin {min_width: REFERENCE_SCREEN_WIDTH, min_height: REFERENCE_SCREEN_HEIGHT},
                ..default()
            },
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::None,
            },
            ..default()
        },
    ));
    commands.spawn((
        UiCamera {},
        Camera2dBundle {
            projection: OrthographicProjection {
                scaling_mode: ScalingMode::AutoMin {min_width: REFERENCE_SCREEN_WIDTH, min_height: REFERENCE_SCREEN_HEIGHT},
                ..default()
            },
            camera: Camera {
                order: 10,
                ..default()
            },
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::None,
            },
            ..default()
        },
        RenderLayers::layer(31),
    ));
}
