// main.rs
#![allow(unused_imports)]
#![allow(dead_code)]

use raylib::prelude::*;
use std::f32::consts::PI;

mod framebuffer;
mod camera;
mod obj;
mod matrix;
mod line;
mod triangle;
mod fragment;
mod light;

use framebuffer::Framebuffer;
use camera::Camera;
use obj::Obj;

use triangle::triangle;
use crate::{light::Light, matrix::{create_model_matrix, create_projection_matrix, create_view_matrix, create_viewport_matrix, multiply_matrix_vector4}};


fn transform(
    vertex: Vector3,
    translation: Vector3,
    scale: f32,
    rotation: Vector3,
    view: &Matrix,
    projection: &Matrix,
    viewport: &Matrix,
) -> Vector3 {
    let model : Matrix = create_model_matrix(translation, scale, rotation);
    let vertex4 = Vector4::new(vertex.x, vertex.y, vertex.z, 1.0);

    let world_transform = multiply_matrix_vector4(&model, &vertex4);
    let view_transform = multiply_matrix_vector4(view, &world_transform);
    let projection_transform = multiply_matrix_vector4(projection, &view_transform);

    // División por w (NDC)
    let ndc = Vector4::new(
        projection_transform.x / projection_transform.w,
        projection_transform.y / projection_transform.w,
        projection_transform.z / projection_transform.w,
        1.0
    );

    // Viewport una sola vez
    let screen = multiply_matrix_vector4(viewport, &ndc);
    Vector3::new(screen.x, screen.y, screen.z)
}

pub fn render(
    framebuffer: &mut Framebuffer,
    translation: Vector3,
    scale: f32,
    rotation: Vector3,
    vertex_array: &[Vector3],
    view: &Matrix,
    projection: &Matrix,
    viewport: &Matrix,
) {
    let light = Light::new(Vector3::new(0.0, 10.0, 0.0));
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = transform(vertex.clone(), translation, scale, rotation, view, projection, viewport);
        transformed_vertices.push(transformed);
    }

    // Primitive Assembly Stage
    let mut triangles = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            triangles.push([
                transformed_vertices[i].clone(),
                transformed_vertices[i + 1].clone(),
                transformed_vertices[i + 2].clone(),
            ]);
        }
    }

    // Rasterization Stage
    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2], &light));
    }

    // Fragment Processing Stage
    for fragment in fragments {
        framebuffer.set_current_color(fragment.color);
        framebuffer.set_pixel(
            fragment.position.x as u32,
            fragment.position.y as u32,
            fragment.depth
        );
    }

}

fn main() {
    let window_width = 1300;
    let window_height = 600;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Wireframe")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let projection = create_projection_matrix(PI/3.0, window_width as f32 / window_height as f32, 0.1, 100.0);
    let viewport = create_viewport_matrix(0.0, 0.0, window_width as f32, window_height as f32);

    let mut framebuffer = Framebuffer::new(window_width as u32, window_height as u32, Color::BLACK);
    framebuffer.set_background_color(Color::new(4, 12, 36, 255));

    let mut translation = Vector3::new(0.0, 0.0, 0.0);
    let scale = 1.0;
    let mut rotation = Vector3::new(0.0, 0.0, 0.0);

    let obj = Obj::load("nave.obj").expect("Failed to load");
    let vtxarray = obj.get_vertex_array();

    let mut camera = Camera::new(
        Vector3::new(0.0, 0.0, 5.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
    );
    //let rotation_speed = PI / 100.0;

    while !window.window_should_close() {
        framebuffer.clear();
//        if window.is_key_down(KeyboardKey::KEY_LEFT)  { camera.orbit( rotation_speed, 0.0); }
  //      if window.is_key_down(KeyboardKey::KEY_RIGHT) { camera.orbit(-rotation_speed, 0.0); }
    //    if window.is_key_down(KeyboardKey::KEY_UP)    { camera.orbit(0.0, -rotation_speed); }
      //  if window.is_key_down(KeyboardKey::KEY_DOWN)  { camera.orbit(0.0,  rotation_speed); }
      camera.process_input(&window);

        let view = camera.get_view_matrix();
        render(&mut framebuffer, translation, scale, rotation, &vtxarray, &view, &projection, &viewport);
        framebuffer.swap_buffers(&mut window, &raylib_thread);
    }
}