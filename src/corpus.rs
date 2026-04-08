use std::fs::read_to_string;

use bevy::{ecs::component::Component, math::Vec2};

#[derive(Component)]
pub struct WordLength {
    x_axis: Vec<f32>,
    y_axis: Vec<f32>,

    // for drawing
    origin: Vec2,
    y_scale: f32,
    plot_length: f32,
    pub vertices: Vec<Vec2>
}

impl WordLength {
    pub fn new(corpus_path: &str, length: f32, y_scale: f32, origin: Vec2) -> Self {
        let corpus_raw: String = read_to_string(corpus_path).unwrap();
        let mut corpus_sanitized: String = String::new();

        for c in corpus_raw.chars() {
            match c {
                '.' | ',' | '?' | '!' | ';' | ':' | '"' | '\n' | '\r' | '\t' => {},
                _ => {
                    corpus_sanitized.push(c);
                }
            }
        }
        
        let words: Vec<&str> = corpus_sanitized.split(" ").collect();
        let y_axis: Vec<f32> = words.iter().map(|s| s.len() as f32).collect();

        let mut x_axis: Vec<f32> = Vec::new();
        let spacing: f32 = length / (y_axis.len() as f32);

        for i in 0..y_axis.len() {
            x_axis.push((i as f32) * spacing);
        }

        let mut vertices: Vec<Vec2> = Vec::new();

        for i in 0..y_axis.len() {
            vertices.push(Vec2::new(x_axis[i] + origin.x, (y_axis[i] * y_scale) + origin.y));
        }

        WordLength {
            x_axis,
            y_axis,

            origin,
            plot_length: length,
            y_scale,
            vertices
        }
    }
}
