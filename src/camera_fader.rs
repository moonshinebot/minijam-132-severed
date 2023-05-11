use bevy::math::vec2;
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy::sprite::Anchor;
use crate::camera_fader::FaderState::{FadeIn, FadeOut};

pub struct CameraFaderPlugin;

impl Plugin for CameraFaderPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(set_up_camera_faders)
            .add_system(apply_fader_alpha);
    }
}

fn set_up_camera_faders(mut commands: Commands,
                        camera_query: Query<Entity, (With<Camera>, Without<CameraFader>)>,
                        render_layer_query: Query<&RenderLayers>) {
    for camera_entity in camera_query.iter() {
        let sprite_entity = commands.spawn(
            SpriteBundle {
                sprite: Sprite {
                    anchor: Anchor::TopLeft,
                    color: Color::BLACK,
                    custom_size: Some(vec2(213.0, 120.0)),
                    ..default()
                },
                transform: Transform::from_xyz(-213.0 / 2.0, 60.0, 100.0),
                ..default()
            }).id();

        // If the camera is configured to a specific RenderLayer, apply that render layer to the
        // fader Sprite, too.
        if let Ok(render_layer) = render_layer_query.get(camera_entity) {
            commands.entity(sprite_entity).insert(render_layer.clone());
        }

        commands
            .entity(camera_entity)
            .insert((
                CameraFader {
                    state: FadeOut,
                    fadeout_amount: 1.0,
                    sprite: sprite_entity,
                },
            ));
    }
}

fn apply_fader_alpha(time: Res<Time>,
                     mut fader_query: Query<&mut CameraFader>,
                     mut sprite_query: Query<&mut Sprite>) {
    for mut fader in fader_query.iter_mut() {
        if fader.state == FadeOut && fader.fadeout_amount < 1.0 {
            fader.fadeout_amount = f32::min(1.0, fader.fadeout_amount + 0.5 * time.delta_seconds());
        } else if fader.state == FadeIn && fader.fadeout_amount > 0.0 {
            fader.fadeout_amount = f32::max(0.0, fader.fadeout_amount - 0.5 * time.delta_seconds());
        } else {
            continue;
        }

        let mut sprite = sprite_query.get_mut(fader.sprite).unwrap();
        sprite.color.set_a(fader.fadeout_amount);
    }
}

#[derive(Component)]
pub struct CameraFader {
    /// Mutate to fade in or out.
    pub state: FaderState,
    fadeout_amount: f32,
    /// The entity containing the Sprite used for fading this camera.
    sprite: Entity,
}

#[derive(PartialEq)]
pub enum FaderState {
    FadeIn,
    FadeOut,
}
