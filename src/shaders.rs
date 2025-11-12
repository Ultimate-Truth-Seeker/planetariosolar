use std::f32::consts::PI;
use raylib::math::*;
use raylib::prelude::*;

use crate::fragment::Fragment;
use crate::uniforms::{Uniforms, color_to_vec3};
use crate::procedural::fbm3;

// -------------------- Material (per-entity) --------------------
#[derive(Clone)]
pub struct Material {
    // L1: two cosine palettes (IQ-style) to blend
    pub pal1_a: Vector3, pub pal1_b: Vector3, pub pal1_c: Vector3, pub pal1_d: Vector3,
    pub pal2_a: Vector3, pub pal2_b: Vector3, pub pal2_c: Vector3, pub pal2_d: Vector3,
    pub pal_mix_radius: f32, // how quickly we mix pal1->pal2 with radius (0..1 region around center)

    // L2: stripes/rings/checker and accent
    pub rings_freq: f32, pub rings_speed: f32,
    pub stripes_angle_deg: f32, pub stripes_freq: f32, pub stripes_speed: f32,
    pub checker_scale: f32,
    pub rings_weight: f32, pub stripes_weight: f32, pub checker_weight: f32,
    pub accent: Vector3, pub accent_strength: f32,

    // L3: noise/perturb
    pub fbm_freq: f32, pub fbm_octaves: u32, pub fbm_lacunarity: f32, pub fbm_gain: f32, pub fbm_time: f32,
    pub sat_base: f32, pub sat_amp: f32,
    pub bri_base: f32, pub bri_amp: f32,

    // L4: scanlines/bloom
    pub scan_intensity: f32,
    pub bloom_strength: f32,
}

impl Material {
    pub fn rocky() -> Self {
        Self {
            pal1_a: Vector3::new(0.55, 0.45, 0.40), pal1_b: Vector3::new(0.30, 0.25, 0.20), pal1_c: Vector3::new(1.0, 1.0, 1.0), pal1_d: Vector3::new(0.00, 0.15, 0.20),
            pal2_a: Vector3::new(0.45, 0.35, 0.30), pal2_b: Vector3::new(0.25, 0.20, 0.15), pal2_c: Vector3::new(1.0, 1.0, 1.0), pal2_d: Vector3::new(0.35, 0.10, 0.20),
            pal_mix_radius: 0.6,
            rings_freq: 12.0, rings_speed: 0.2,
            stripes_angle_deg: 25.0, stripes_freq: 18.0, stripes_speed: 0.2,
            checker_scale: 6.0,
            rings_weight: 0.15, stripes_weight: 0.10, checker_weight: 0.05,
            accent: Vector3::new(0.20, 0.16, 0.12), accent_strength: 0.25,
            fbm_freq: 5.0, fbm_octaves: 5, fbm_lacunarity: 2.0, fbm_gain: 0.5, fbm_time: 0.25,
            sat_base: 0.7, sat_amp: 0.35,
            bri_base: 0.9, bri_amp: 0.25,
            scan_intensity: 0.06, bloom_strength: 0.08,
        }
    }
    pub fn gaseous() -> Self {
        Self {
            pal1_a: Vector3::new(0.15, 0.35, 0.60), pal1_b: Vector3::new(0.20, 0.35, 0.40), pal1_c: Vector3::new(1.0, 1.0, 1.0), pal1_d: Vector3::new(0.05, 0.10, 0.20),
            pal2_a: Vector3::new(0.75, 0.60, 0.30), pal2_b: Vector3::new(0.30, 0.30, 0.20), pal2_c: Vector3::new(1.0, 1.0, 1.0), pal2_d: Vector3::new(0.20, 0.15, 0.10),
            pal_mix_radius: 0.9,
            rings_freq: 24.0, rings_speed: 0.5,
            stripes_angle_deg: 12.0, stripes_freq: 48.0, stripes_speed: 0.9,
            checker_scale: 10.0,
            rings_weight: 0.45, stripes_weight: 0.45, checker_weight: 0.10,
            accent: Vector3::new(0.08, 0.65, 0.95), accent_strength: 0.35,
            fbm_freq: 3.5, fbm_octaves: 4, fbm_lacunarity: 2.0, fbm_gain: 0.55, fbm_time: 0.35,
            sat_base: 0.9, sat_amp: 0.20,
            bri_base: 0.95, bri_amp: 0.30,
            scan_intensity: 0.05, bloom_strength: 0.12,
        }
    }
    pub fn ring() -> Self {
        Self {
            pal1_a: Vector3::new(0.65, 0.60, 0.55), pal1_b: Vector3::new(0.20, 0.20, 0.20), pal1_c: Vector3::new(1.0, 1.0, 1.0), pal1_d: Vector3::new(0.10, 0.10, 0.10),
            pal2_a: Vector3::new(0.55, 0.50, 0.45), pal2_b: Vector3::new(0.15, 0.15, 0.15), pal2_c: Vector3::new(1.0, 1.0, 1.0), pal2_d: Vector3::new(0.20, 0.20, 0.20),
            pal_mix_radius: 0.8,
            rings_freq: 90.0, rings_speed: 0.0,
            stripes_angle_deg: 0.0, stripes_freq: 0.0, stripes_speed: 0.0,
            checker_scale: 14.0,
            rings_weight: 0.85, stripes_weight: 0.0, checker_weight: 0.05,
            accent: Vector3::new(0.9, 0.85, 0.7), accent_strength: 0.12,
            fbm_freq: 8.0, fbm_octaves: 3, fbm_lacunarity: 2.0, fbm_gain: 0.5, fbm_time: 0.0,
            sat_base: 0.85, sat_amp: 0.10,
            bri_base: 0.95, bri_amp: 0.10,
            scan_intensity: 0.04, bloom_strength: 0.08,
        }
    }
    pub fn star() -> Self {
        Self {
            pal1_a: Vector3::new(1.0, 0.95, 0.85), pal1_b: Vector3::new(0.1, 0.1, 0.1), pal1_c: Vector3::new(1.0, 1.0, 1.0), pal1_d: Vector3::new(0.0, 0.33, 0.67),
            pal2_a: Vector3::new(1.0, 0.9, 0.7),  pal2_b: Vector3::new(0.2, 0.1, 0.0), pal2_c: Vector3::new(1.0, 1.0, 1.0), pal2_d: Vector3::new(0.15, 0.33, 0.5),
            pal_mix_radius: 1.0,
            rings_freq: 0.0, rings_speed: 0.0,
            stripes_angle_deg: 0.0, stripes_freq: 0.0, stripes_speed: 0.0,
            checker_scale: 0.0,
            rings_weight: 0.0, stripes_weight: 0.0, checker_weight: 0.0,
            accent: Vector3::new(1.0, 0.95, 0.85), accent_strength: 0.05,
            fbm_freq: 1.8, fbm_octaves: 5, fbm_lacunarity: 2.0, fbm_gain: 0.54, fbm_time: 0.8,
            sat_base: 1.0, sat_amp: 0.05,
            bri_base: 1.1, bri_amp: 0.35,
            scan_intensity: 0.03, bloom_strength: 0.35,
        }
    }
    pub fn moon() -> Self {
        Self {
            pal1_a: Vector3::new(0.6, 0.6, 0.6), pal1_b: Vector3::new(0.2, 0.2, 0.2), pal1_c: Vector3::new(1.0, 1.0, 1.0), pal1_d: Vector3::new(0.2, 0.2, 0.2),
            pal2_a: Vector3::new(0.45, 0.45, 0.45), pal2_b: Vector3::new(0.15, 0.15, 0.15), pal2_c: Vector3::new(1.0, 1.0, 1.0), pal2_d: Vector3::new(0.3, 0.3, 0.3),
            pal_mix_radius: 0.6,
            rings_freq: 0.0, rings_speed: 0.0,
            stripes_angle_deg: 0.0, stripes_freq: 0.0, stripes_speed: 0.0,
            checker_scale: 8.0,
            rings_weight: 0.0, stripes_weight: 0.0, checker_weight: 0.05,
            accent: Vector3::new(0.35, 0.35, 0.35), accent_strength: 0.15,
            fbm_freq: 6.0, fbm_octaves: 5, fbm_lacunarity: 2.0, fbm_gain: 0.5, fbm_time: 0.25,
            sat_base: 0.5, sat_amp: 0.2,
            bri_base: 0.9, bri_amp: 0.15,
            scan_intensity: 0.02, bloom_strength: 0.05,
        }
    }
}

// -------------------- Utilities --------------------
fn rotate2(p: Vector2, angle: f32) -> Vector2 {
    let (s, c) = angle.sin_cos();
    Vector2::new(c * p.x - s * p.y, s * p.x + c * p.y)
}
fn uv_from_pos(pos: Vector2, res: Vector2) -> Vector2 {
    Vector2::new(pos.x / res.x - 0.5, pos.y / res.y - 0.5)
}
fn pattern_checker(uv: Vector2, scale: f32, angle: f32) -> f32 {
    let suv = rotate2(uv * scale, angle);
    let cx = suv.x.floor();
    let cy = suv.y.floor();
    let parity = ((cx as i32) + (cy as i32)) & 1;
    if parity == 0 { 0.0 } else { 1.0 }
}
fn pattern_rings(uv: Vector2, freq: f32, speed: f32, time: f32) -> f32 {
    let r = (uv.x * uv.x + uv.y * uv.y).sqrt();
    let phase = r * freq - time * speed;
    ((phase * std::f32::consts::TAU).sin() * 0.5 + 0.5) // 0..1
}
fn pattern_stripes(uv: Vector2, angle: f32, freq: f32, speed: f32, time: f32) -> f32 {
    let dir = Vector2::new(angle.cos(), angle.sin());
    let t = uv.x * dir.x + uv.y * dir.y;
    let phase = t * freq - time * speed;
    (phase.sin() * 0.5 + 0.5) // 0..1
}
fn overlay_scanlines(pos: Vector2, intensity: f32) -> f32 {
    let s = ((pos.y * std::f32::consts::PI / 2.0).sin() * 0.5 + 0.5);
    1.0 - intensity * (1.0 - s) // 1.0 → sin efecto
}
fn palette_cosine(t: f32, a: Vector3, b: Vector3, c: Vector3, d: Vector3) -> Vector3 {
    Vector3::new(
        a.x + b.x * ( (6.28318 * (c.x * t + d.x)).cos() ),
        a.y + b.y * ( (6.28318 * (c.y * t + d.y)).cos() ),
        a.z + b.z * ( (6.28318 * (c.z * t + d.z)).cos() ),
    )
}

// -------------------- Fragment Shader (4 capas) --------------------
/// L1: albedo/paleta | L2: bandas/estrías | L3: ruido/perturbación | L4: scanlines/brillo
/// `layers` (l1,l2,l3,l4) y `mat` definen cómo luce cada entidad.
pub fn fragment_shader(fragment: &Fragment, u: &Uniforms, layers: (bool, bool, bool, bool), mat: &Material) -> Vector3 {
    let (l1, l2, l3, l4) = layers;
    let pos = Vector2::new(fragment.position.x, fragment.position.y);               // pixel coords
    let res = u.resolution;
    let uv = uv_from_pos(pos, res);            // [-.5, .5]
    let time = u.time;

    // Base color del fragmento (0..1)
    let base = fragment.color;
    let mut col = base;
    // ... arriba no cambies nada ...

    // ✅ Normaliza si entró en escala 0..255 (defensa)
    if col.x > 1.0 || col.y > 1.0 || col.z > 1.0 {
        col = Vector3::new(col.x / 255.0, col.y / 255.0, col.z / 255.0);
    }

    // ---- L1: Paletas ----
    if l1 {
        let ang = (uv.y).atan2(uv.x); // [-pi,pi]
        let t = ((ang + time * 0.2) / std::f32::consts::TAU + 0.5) % 1.0;
        let pal1 = palette_cosine(t, mat.pal1_a, mat.pal1_b, mat.pal1_c, mat.pal1_d);
        let pal2 = palette_cosine(t, mat.pal2_a, mat.pal2_b, mat.pal2_c, mat.pal2_d);
        let r = (uv.x * uv.x + uv.y * uv.y).sqrt();
        let mix_r = (r / mat.pal_mix_radius).clamp(0.0, 1.0);
        let pal = pal1 * (1.0 - mix_r) + pal2 * mix_r;
        col = col * 0.4 + pal * 0.6;
    }

    // ---- L2: Bandas / Estrías ----
    if l2 {
        let rings = if mat.rings_weight > 0.0 { pattern_rings(uv, mat.rings_freq, mat.rings_speed, time) } else { 0.0 };
        let stripes = if mat.stripes_weight > 0.0 { pattern_stripes(uv, mat.stripes_angle_deg.to_radians(), mat.stripes_freq, mat.stripes_speed, time) } else { 0.0 };
        let checker = if mat.checker_weight > 0.0 { pattern_checker(uv, mat.checker_scale, time * 0.15) } else { 0.0 };
        let bands = (mat.rings_weight * rings + mat.stripes_weight * stripes + mat.checker_weight * checker)
            .clamp(0.0, 1.0);
        col = col * (1.0 - mat.accent_strength * bands) + mat.accent * (mat.accent_strength * bands);
    }

    // ---- L3: Ruido / Perturbación ----
    if l3 {
        let f = fbm3(Vector3::new(uv.x * mat.fbm_freq, uv.y * mat.fbm_freq, time * mat.fbm_time), mat.fbm_octaves, mat.fbm_lacunarity, mat.fbm_gain) * 0.5 + 0.5;
        let sat = mat.sat_base + mat.sat_amp * f;
        let bri = mat.bri_base + mat.bri_amp * f;
        col = Vector3::new(col.x * sat, col.y * sat, col.z * sat) * bri;
    }

    // ---- L4: Scanlines / Bloom ----
    if l4 {
        let scan = overlay_scanlines(pos, mat.scan_intensity);
        let boost = (col.x.max(col.y).max(col.z)).powf(2.0) * mat.bloom_strength;
        col = col * scan + Vector3::new(boost, boost, boost);
    }

    // Gamma ligera
    // ✅ Clamp antes de la gamma (evita quemar a blanco)
    col = Vector3::new(
        col.x.clamp(0.0, 1.0),
        col.y.clamp(0.0, 1.0),
        col.z.clamp(0.0, 1.0),
    );

    // Gamma ligera
    Vector3::new(
        col.x.powf(1.0 / 1.2),
        col.y.powf(1.0 / 1.2),
        col.z.powf(1.0 / 1.2),
    )
}