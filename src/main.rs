use bevy::{DefaultPlugins, app::{App, PluginGroup, Startup, Update}, asset::Assets, camera::{Camera2d, ClearColor, OrthographicProjection, Projection}, color::Color, ecs::{query::With, system::{Commands, Res, ResMut, Single}}, input::{ButtonInput, keyboard::KeyCode}, math::Vec2, prelude::Mesh, sprite_render::ColorMaterial, transform::components::Transform, window::{MonitorSelection, Window, WindowMode, WindowPlugin}};

use crate::{corpus::BigramFreq, discriminate::discriminate_language};

mod corpus;
mod discriminate;

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
        .add_systems(Update, move_around)
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

    let french_corpus: BigramFreq = BigramFreq::new("corpora/french.txt", "french", Vec2::new(-850.0, -250.0), Vec2::new(500.0, 500.0));
    let spanish_corpus: BigramFreq = BigramFreq::new("corpora/spanish.txt", "spanish", Vec2::new(-225.0, -250.0), Vec2::new(500.0, 500.0));
    let italian_corpus: BigramFreq = BigramFreq::new("corpora/italian.txt", "italian", Vec2::new(375.0, -250.0), Vec2::new(500.0, 500.0));
    let english_corpus: BigramFreq = BigramFreq::new("corpora/english.txt", "english", Vec2::new(1070.0, -250.0), Vec2::new(500.0, 500.0));
    let german_corpus: BigramFreq = BigramFreq::new("corpora/german.txt", "german", Vec2::new(1695.0, -250.0), Vec2::new(500.0, 500.0));
    let dutch_corpus: BigramFreq = BigramFreq::new("corpora/dutch.txt", "dutch", Vec2::new(2295.0, -250.0), Vec2::new(500.0, 500.0));
    let polish_corpus: BigramFreq = BigramFreq::new("corpora/polish.txt", "polish", Vec2::new(3290.0, -250.0), Vec2::new(500.0, 500.0));
    let slovak_corpus: BigramFreq = BigramFreq::new("corpora/slovak.txt", "slovak", Vec2::new(3915.0, -250.0), Vec2::new(500.0, 500.0));
    let indonesian_corpus: BigramFreq = BigramFreq::new("corpora/indonesian.txt", "indonesian", Vec2::new(5210.0, -250.0), Vec2::new(500.0, 500.0));
    let tagalog_corpus: BigramFreq = BigramFreq::new("corpora/tagalog.txt", "tagalog", Vec2::new(5835.0, -250.0), Vec2::new(500.0, 500.0));

    french_corpus.spawn(&mut commands, &mut meshes, &mut materials);
    spanish_corpus.spawn(&mut commands, &mut meshes, &mut materials);
    italian_corpus.spawn(&mut commands, &mut meshes, &mut materials);
    english_corpus.spawn(&mut commands, &mut meshes, &mut materials);
    german_corpus.spawn(&mut commands, &mut meshes, &mut materials);
    dutch_corpus.spawn(&mut commands, &mut meshes, &mut materials);
    polish_corpus.spawn(&mut commands, &mut meshes, &mut materials);
    slovak_corpus.spawn(&mut commands, &mut meshes, &mut materials);
    indonesian_corpus.spawn(&mut commands, &mut meshes, &mut materials);
    tagalog_corpus.spawn(&mut commands, &mut meshes, &mut materials);

    let corpus_en: BigramFreq = BigramFreq::new("sample.txt", "sample", Vec2::new(0.0, 0.0), Vec2::new(0.0, 0.0));

    let correspondance: String = discriminate_language(&corpus_en, &[french_corpus, english_corpus, german_corpus, dutch_corpus, italian_corpus, spanish_corpus, polish_corpus, slovak_corpus, indonesian_corpus, tagalog_corpus]);
    println!("closest language: {}", correspondance);
}

fn move_around(mut camera_transform: Single<&mut Transform, With<Camera2d>>, buttons: Res<ButtonInput<KeyCode>>) {
    if buttons.just_pressed(KeyCode::ArrowRight) {
        camera_transform.translation.x = (camera_transform.translation.x + 1920.0).min(5760.0);
    }

    if buttons.just_pressed(KeyCode::ArrowLeft) {
        camera_transform.translation.x = (camera_transform.translation.x - 1920.0).max(0.0);
    }
}
