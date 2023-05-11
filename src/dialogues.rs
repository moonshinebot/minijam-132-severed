use bevy::app::{App, Plugin};
use bevy::prelude::{Commands, Component, Entity, Query, Res, ResMut, Resource, Time, Timer, TimerMode};
use std::io;
use std::io::Write;

pub struct DialoguePlugin;

impl Plugin for DialoguePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(DialogueBox::default())
            .insert_resource(GreetTimer(Timer::from_seconds(0.1, TimerMode::Repeating)))
            .add_system(print_messages);
    }
}

fn print_messages(mut commands: Commands, query: Query<(Entity, &Message)>) {
    for (entity, message) in &query {
        println!("{}", message.0);
        io::stdout().flush().expect("Flushing failed!");
        commands.entity(entity).despawn()
    }
}


#[derive(Resource)]
struct DialogueBox {
    pub messages: Vec<String>,
}

impl Default for DialogueBox {
    fn default() -> DialogueBox {
        DialogueBox { messages: vec![] }
    }
}

#[derive(Component)]
pub struct Message(String);

#[derive(Resource)]
pub struct GreetTimer(Timer);

fn add_messages(mut commands: Commands) {
    commands.spawn(Message("This is a message 1".to_string()));
    commands.spawn(Message("This is a message 2".to_string()));
    commands.spawn(Message("This is a message 3".to_string()));
    commands.spawn(Message("This is a message 4".to_string()));
}

fn play_dialogue(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Message>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("Hallo {}!", name.0);
        }
    }
}
