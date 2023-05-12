use bevy::math::vec3;
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use camera_fader::{FaderState};
use crate::camera_fader::CameraFader;

mod stage;
mod camera;
mod assistant;
mod camera_fader;

fn main() {
    App::new()
        .init_resource::<StartFadeInTimer>()
        .add_plugins(DefaultPlugins.set(
            ImagePlugin::default_nearest(),
        ))
        .add_plugin(bevy_framepace::FramepacePlugin)
        .add_plugin(camera_fader::CameraFaderPlugin)
        .add_plugin(camera::MainCameraPlugin)
        .add_startup_system(stage::add_stage)
        .add_startup_system(setup)
        .add_system(begin_game)
        .run();
}

#[derive(Component)]
struct OverlayText;

#[derive(Resource)]
struct StartFadeInTimer(Timer);

impl Default for StartFadeInTimer {
    fn default() -> Self {
        StartFadeInTimer(Timer::from_seconds(3.0, TimerMode::Once))
    }
}

fn setup(mut commands: Commands,
         asset_server: Res<AssetServer>,
         mut framepace_settings: ResMut<bevy_framepace::FramepaceSettings>) {
    framepace_settings.limiter = bevy_framepace::Limiter::from_framerate(30.0);

    let font = asset_server.load("fonts/Pixel64_v1.2.ttf");
    let text_style = TextStyle {
        font,
        font_size: 64.0,
        color: Color::WHITE,
    };
    commands.spawn((
        Text2dBundle {
            transform: Transform::from_scale(vec3(0.1, 0.1, 0.1)),
            text: Text::from_section("This is the story of Bob ...", text_style)
                .with_alignment(TextAlignment::Center),
            ..default()
        },
        RenderLayers::layer(31),
        OverlayText {}
    ));
}

fn begin_game(time: Res<Time>,
              mut start_timer: ResMut<StartFadeInTimer>,
              mut fader_query: Query<&mut CameraFader>) {
    if start_timer.0.tick(time.delta()).just_finished() {
        for mut fader in fader_query.iter_mut() {
            fader.state = FaderState::FadeIn;
        }
    }
}
