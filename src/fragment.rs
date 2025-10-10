#![allow(dead_code)]

use raylib::{color::Color, math::{Vector2, Vector3}};

pub struct Fragment {
    pub position: Vector2,
    pub color: Color,
    pub depth: f32,
}

impl Fragment {
    pub fn new(x: f32, y: f32, color: Color, depth: f32) -> Self {
        Fragment {
            position: Vector2::new(x, y),
            color,
            depth,
        }
    }
}