use bevy::{DefaultPlugins, app::{App, PluginGroup, Startup}, asset::Assets, camera::{Camera2d, ClearColor, OrthographicProjection, Projection}, color::Color, ecs::{system::{Commands, ResMut}}, math::Vec2, prelude::Mesh, sprite_render::ColorMaterial, transform::components::Transform, window::{MonitorSelection, Window, WindowMode, WindowPlugin}};

use crate::corpus::{BigramFreq};

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
        // .add_systems(Update, move_around)
        .run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn((
        Camera2d,
        Transform::from_xyz(0.0, 0.0, 0.0),
        Projection::from(OrthographicProjection {
            scale: 1.0,
            ..OrthographicProjection::default_2d()
        })
    ));

    let lang: &str = "tagalog";
    let french_corpus: BigramFreq = BigramFreq::new(format!("corpora/{}.txt", lang).as_str(), lang, Vec2::new(-825.0, -400.0), Vec2::new(750.0, 750.0));
    let english_corpus: BigramFreq = BigramFreq::new("corpora/english.txt", "english", Vec2::new(100.0, -400.0), Vec2::new(750.0, 750.0));

    french_corpus.spawn(&mut commands, &mut meshes, &mut materials);
    english_corpus.spawn(&mut commands, &mut meshes, &mut materials);
}

// fn move_around(mut evr_motion: MessageReader<MouseMotion>, buttons: Res<ButtonInput<MouseButton>>, mut cameras: Query<(&mut Transform, &mut Projection), With<Camera2d>>, mw: Res<AccumulatedMouseScroll>) {
//     for (mut transform, proj) in cameras.iter_mut() {
//         if buttons.pressed(MouseButton::Left) {
//             for ev in evr_motion.read() {
//                 transform.translation.x -= ev.delta.x;
//                 transform.translation.y += ev.delta.y;
//             }
//         }
//
//         match *proj.into_inner() {
//             Projection::Orthographic(ref mut orth) => {
//                 let zoom = -mw.delta.y * 0.1;
//                 orth.scale *= 1.0 + zoom;
//             },
//             _ => unreachable!()
//         }
//     }
// }
