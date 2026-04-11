use std::{f32::consts::PI, fs::read_to_string};

use bevy::{asset::{Assets, Handle}, color::Color, ecs::{component::Component, system::{Commands, ResMut}}, math::{Quat, Vec2, primitives::Rectangle}, mesh::Mesh2d, prelude::Mesh, sprite::Text2d, sprite_render::{ColorMaterial, MeshMaterial2d}, text::TextFont, transform::components::Transform};
use diacritics::remove_diacritics;

#[derive(Component)]
pub struct BigramFreq {
    name: String,
    bigrams: Vec<Vec<f32>>,
    max_freq: f32,

    // for drawing
    origin: Vec2,
    plot_dimensions: Vec2,
}

impl BigramFreq {
    pub fn new(corpus_path: &str, name: &str, origin: Vec2, plot_dimensions: Vec2) -> Self {
        let corpus_raw: String = read_to_string(corpus_path).unwrap();
        let corpus_sanitized: String = sanitize_text(&corpus_raw);
        let (bigrams, max_freq): (Vec<Vec<f32>>, f32) = get_bigrams(&corpus_sanitized);
        
        Self {
            name: name.to_string(),
            bigrams,
            max_freq,
            
            origin,
            plot_dimensions
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn freq_at(&self, i: usize, j: usize) -> f32 {
        self.bigrams[i][j]
    }

    pub fn spawn(&self, commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<ColorMaterial>>) {
        let axis_thickness: f32 = 10.0;
        let rect_width: f32 = self.plot_dimensions.x / 27.0;
        let rect_height: f32 = self.plot_dimensions.y / 27.0;
        
        let axis_knudge: f32 = 3.0;
        let axis_x: Handle<Mesh> = meshes.add(Rectangle::new(self.plot_dimensions.x + axis_thickness * axis_knudge, axis_thickness));
        let axis_y: Handle<Mesh> = meshes.add(Rectangle::new(axis_thickness, self.plot_dimensions.y + axis_thickness * axis_knudge));
        let axis_material = materials.add(Color::srgb_u8(255, 255, 255));

        commands.spawn((
            Mesh2d(axis_x),
            MeshMaterial2d(axis_material.clone()),
            Transform::from_xyz((self.plot_dimensions.x - axis_thickness * axis_knudge) / 2.0 + self.origin.x + axis_thickness, self.origin.y - (axis_thickness / 2.0) - 5.0, 0.0)
        ));
        commands.spawn((
            Mesh2d(axis_y),
            MeshMaterial2d(axis_material),
            Transform::from_xyz(self.origin.x - (axis_thickness / 2.0) - 5.0, (self.plot_dimensions.y - axis_thickness * axis_knudge) / 2.0 + self.origin.y + axis_thickness, 0.0)
        ));

        let rect: Handle<Mesh> = meshes.add(Rectangle::new(rect_width, rect_height));
        let max_freq: f32 = self.max_freq;

        for i in 0..27 {
            for j in 0..27 {
                let freq: f32 = self.bigrams[j][i];
                let t: f32 = freq / max_freq;
                
                let rect_material = materials.add(Color::srgba(0.949, 0.047, 0.317, t));

                commands.spawn((
                    Mesh2d(rect.clone()),
                    MeshMaterial2d(rect_material),
                    Transform::from_xyz(((j as f32)*rect_width + (rect_width / 2.0)) + self.origin.x, ((i as f32)*rect_height + (rect_height / 2.0)) + self.origin.y, 0.0)
                ));
            }
        }

        for i in 0..27 {
            let letter: char = if i == 26 {'_'} else {(i as u8 + 97) as char};

            commands.spawn((
                Text2d::new(letter),
                TextFont {
                    font_size: rect_height,
                    ..Default::default()
                },
                Transform::from_xyz(((i as f32)*rect_width + (rect_width / 2.0)) + self.origin.x, self.origin.y + self.plot_dimensions.y + (rect_height / 2.0), 0.0)
            ));
            commands.spawn((
                Text2d::new(letter),
                TextFont {
                    font_size: rect_height,
                    ..Default::default()
                },
                Transform::from_xyz(self.origin.x + self.plot_dimensions.x + (rect_width / 2.0), ((i as f32)*rect_height + (rect_height / 2.0)) + self.origin.y, 0.0).with_rotation(Quat::from_rotation_z(-PI / 2.0))
            ));
        }

        commands.spawn((
            Text2d::new(self.name.clone()),
            TextFont {
                font_size: rect_height,
                ..Default::default()
            },
            Transform::from_xyz(self.origin.x - rect_height - 5.0, self.origin.y + (self.plot_dimensions.y) / 2.0, 0.0).with_rotation(Quat::from_rotation_z(PI / 2.0))
        ));
    }
}

pub fn sanitize_text(txt: &str) -> String {
    let txt: String = remove_diacritics(txt);
    let mut text_sanitized: String = String::new();
    let mut is_last_space: bool = true;

    for c in txt.chars() {
        if !(c.is_ascii_alphabetic() || (c == ' ' && !is_last_space)) {
            continue;
        }

        is_last_space = c == ' ';

        text_sanitized.push(c.to_ascii_lowercase());
    }

    text_sanitized.trim_ascii_end().to_string()
}

pub fn get_bigrams(txt: &str) -> (Vec<Vec<f32>>, f32) {
    let txt_chars: Vec<char> = txt.chars().collect();
    let mut bigram_count: Vec<Vec<u32>> = vec![vec![0; 27]; 27];

    if txt_chars.len() < 2 {
        return (vec![vec![0.0; 27]; 27], 0.0);
    }

    for i in 0..(txt_chars.len() - 1) {
        let curr: char = txt_chars[i];
        let next: char = txt_chars[i+1];

        let curr_idx: usize = if curr == ' ' {26} else {(curr as usize) - ('a' as usize)};
        let next_idx: usize = if next == ' ' {26} else {(next as usize) - ('a' as usize)};

        bigram_count[curr_idx][next_idx] += 1;
    }

    let total_bigrams: f32 = (txt_chars.len() - 1) as f32;
    let mut bigram_freq: Vec<Vec<f32>> = vec![vec![0.0; 27]; 27];

    let mut max_freq: f32 = 0.0;

    for i in 0..27 {
        for j in 0..27 {
            let freq: f32 = (bigram_count[i][j] as f32) / total_bigrams;

            max_freq = max_freq.max(freq);
            
            bigram_freq[i][j] = freq;
        }
    }

    (bigram_freq, max_freq)
}
