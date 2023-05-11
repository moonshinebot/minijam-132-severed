use bevy::prelude::*;
use bevy::render::view::RenderLayers;

/// A convenience bundle for adding a sprite that should be visible on the main camera.
/// Use [MainCameraSpriteBundle::from_asset] to create the bundle in a single line (with the help of the asset server).
#[derive(Bundle)]
pub struct MainCameraSpriteBundle {
    #[bundle]
    sprite: SpriteBundle,
    render_layer: RenderLayers,
}

impl Default for MainCameraSpriteBundle {
    fn default() -> Self {
        Self {
            sprite: SpriteBundle::default(),
            render_layer: RenderLayers::layer(0),
        }
    }
}
