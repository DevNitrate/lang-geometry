use std::fs::read_to_string;

use bevy::{ecs::component::Component, math::Vec2};
use diacritics::remove_diacritics;

#[derive(Component)]
pub struct BigramFreq {
    bigrams: Vec<Vec<f32>>,

    // for drawing
    origin: Vec2,
    plot_dimensions: Vec2,
}

impl BigramFreq {
    pub fn new(corpus_path: &str, origin: Vec2, plot_dimensions: Vec2) -> Self {
        let corpus_raw: String = read_to_string(corpus_path).unwrap();
        let corpus_sanitized: String = sanitize_text(&corpus_raw);
        
        Self {
            bigrams: get_bigrams(&corpus_sanitized),
            origin,
            plot_dimensions
        }
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

pub fn get_bigrams(txt: &str) -> Vec<Vec<f32>> {
    let txt_chars: Vec<char> = txt.chars().collect();
    let mut bigram_count: Vec<Vec<u32>> = vec![vec![0; 27]; 27];

    if txt_chars.len() < 2 {
        return vec![vec![0.0; 27]; 27];
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

    for i in 0..27 {
        for j in 0..27 {
            bigram_freq[i][j] = (bigram_count[i][j] as f32) / total_bigrams;
        }
    }

    bigram_freq
}
