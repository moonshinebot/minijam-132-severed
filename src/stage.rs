use bevy::prelude::*;

#[derive(Component)]
struct Stage;

#[derive(Component)]
struct Audience;

pub fn add_stage(mut commands: Commands,
                 assets: Res<AssetServer>) {
    commands.spawn((
        Stage {},
        SpriteBundle {
            texture: assets.load("sprites/magic-stage.png"),
            ..default()
        },
    ));

    commands.spawn((
        Audience {},
        SpriteBundle {
            texture: assets.load("sprites/audience-seats.png"),
            ..default()
        },
    ));
}
