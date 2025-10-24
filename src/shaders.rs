use std::f32::consts::PI;

use raylib::math::*;

use crate::fragment::{Fragment};
// shader.rs (o donde tengas tu fragment shader)
use raylib::prelude::*;
use crate::uniforms::{Uniforms, color_to_vec3};

fn rotate2(p: Vector2, angle: f32) -> Vector2 {
    let (s, c) = angle.sin_cos();
    Vector2::new(c*p.x - s*p.y, s*p.x + c*p.y)
}

// uv en [-.5, .5] centrado
fn uv_from_pos(pos: Vector2, res: Vector2) -> Vector2 {
    Vector2::new(
        pos.x / res.x - 0.5,
        pos.y / res.y - 0.5
    )
}

// Tablero (checker) con rotación y escala
fn pattern_checker(uv: Vector2, scale: f32, angle: f32) -> f32 {
    let suv = rotate2(uv * scale, angle);
    let cx = suv.x.floor();
    let cy = suv.y.floor();
    let parity = ((cx as i32) + (cy as i32)) & 1;
    if parity == 0 { 0.0 } else { 1.0 }
}

// Anillos concéntricos animados
fn pattern_rings(uv: Vector2, freq: f32, speed: f32, time: f32) -> f32 {
    let r = (uv.x*uv.x + uv.y*uv.y).sqrt(); // distancia al centro
    let phase = r * freq - time * speed;
    ((phase * std::f32::consts::TAU).sin() * 0.5 + 0.5) // 0..1
}

// Líneas (stripes) con dirección (angulo), frecuencia y animación
fn pattern_stripes(uv: Vector2, angle: f32, freq: f32, speed: f32, time: f32) -> f32 {
    let dir = Vector2::new(angle.cos(), angle.sin());
    let t = uv.x * dir.x + uv.y * dir.y;
    let phase = t * freq - time * speed;
    (phase.sin() * 0.5 + 0.5) // 0..1
}

// Scanlines horizontales estilo CRT (overlay sutil)
fn overlay_scanlines(pos: Vector2, intensity: f32) -> f32 {
    // Intensidad modulada por y-pixel
    let s = ((pos.y * std::f32::consts::PI / 2.0).sin() * 0.5 + 0.5);
    1.0 - intensity * (1.0 - s) // 1.0 → sin efecto
}

// receives fragment -> returns color
pub fn fragment_shader(fragment: &Fragment, u: &Uniforms) -> Vector3 {
    let pos = fragment.position;         // píxeles (x, y)
    let res = u.resolution;              // resolución ventana
    let uv = uv_from_pos(Vector2::new(pos.x, pos.y), res);      // uv centrado [-.5, .5]
    let base = fragment.color; // color base del triángulo (0..1)

    let stripes = pattern_stripes(uv, 45f32.to_radians(), 35.0, 1.2, u.time);
    let patt = (stripes).clamp(0.0, 1.0);

    // Mapear patrón a una paleta simple (dos tonos) o a un gradiente
    // Opción A: mezclar el base con un color vibrante según patt
    let accent = Vector3::new(0.1, 0.8, 1.0); // cian brillante
    let mut col = base * (1.0 - patt) + accent * patt;
    let scan = overlay_scanlines(Vector2::new(pos.x, pos.y), 0.12); // 0..1 multiplicativo, 0.12 intensidad
    col *= scan;

    // Gamma “suave”
    col = Vector3::new(col.x.powf(1.0/1.2), col.y.powf(1.0/1.2), col.z.powf(1.0/1.2));

    col

}