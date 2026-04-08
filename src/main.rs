use bevy::{DefaultPlugins, app::{App, PluginGroup, Startup, Update}, camera::{Camera2d, ClearColor}, color::Color, ecs::system::{Commands, Query}, gizmos::gizmos::Gizmos, math::Vec2, window::{MonitorSelection, Window, WindowMode, WindowPlugin}};

use crate::corpus::WordLength;

mod corpus;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .insert_resource(ClearColor(Color::srgb_u8(0, 7, 20)))
        .add_systems(Startup, setup)
        .add_systems(Update, draw_word_length)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn(WordLength::new("corpus_en.txt", 1600.0, 10.0, Vec2::new(-800.0, -400.0)));
}

fn draw_word_length(mut gizmos: Gizmos, word_lengths: Query<&WordLength>) {
    for word_length in word_lengths.iter() {
        gizmos.linestrip_2d(word_length.vertices.clone(), Color::srgb_u8(181, 61, 61));
    }
}
