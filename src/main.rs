use bevy::{DefaultPlugins, app::{App, PluginGroup, Startup}, camera::{Camera2d, ClearColor}, color::Color, ecs::system::Commands, window::{MonitorSelection, Window, WindowMode, WindowPlugin}};

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
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
