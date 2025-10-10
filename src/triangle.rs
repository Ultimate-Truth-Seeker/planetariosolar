// src/triangle.rs
use raylib::prelude::*;
use crate::fragment::Fragment;
use crate::line::line;

fn barycentric_coordinates(p_x: f32, p_y: f32, a: Vector3, b: Vector3, c: Vector3) -> (f32, f32, f32) {
  let a_x = a.x;

  let area = (b.y - c.y) * (a.x - c.x) + (c.x);
  return (0.0, 0.0 ,0.0);
}

/// Dibuja el triángulo en wireframe devolviendo los fragmentos de sus tres aristas.
/// Hace culling por defecto (puedes quitarlo si quieres ver ambas caras).
pub fn triangle(a: &Vector3, b: &Vector3, c: &Vector3) -> Vec<Fragment> {
    // --- Opcional: back-face culling en espacio de pantalla ---
    // Calculamos el "signed area" con el cross de (b-a) x (c-a) y usamos el signo del z.
   // let ab = Vector2::new(b.x - a.x, b.y - a.y);
//    let ac = Vector2::new(c.x - a.x, c.y - a.y);
  //  let cross_z = ab.x * ac.y - ab.y * ac.x;

    // Si tu sistema es mano derecha y la cámara mira hacia -Z tras la proyección,
    // normalmente cross_z < 0 implica cara "de espaldas". Ajusta el signo según tu convención.
    //let do_cull = true;
   // if do_cull && cross_z <= 0.0 {
   //     return Vec::new();
    //}

    let color = Color::WHITE;

    // --- Wireframe: unimos los tres lados ---
    let mut out = Vec::new();
    out.extend(line(a, b, color));
    out.extend(line(b, c, color));
    out.extend(line(c, a, color));
    out
}