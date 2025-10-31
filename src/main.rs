// main.rs
#![allow(unused_imports)]
#![allow(dead_code)]
#[inline]
fn rotate_y(v: Vector3, ang: f32) -> Vector3 {
    let (s, c) = ang.sin_cos();
    Vector3::new(c*v.x + 0.0*v.y + -s*v.z, v.y, s*v.x + 0.0*v.y + c*v.z)
}

use raylib::prelude::*;
use std::f32::consts::PI;
use std::time::Instant;

mod framebuffer;
mod camera;
mod obj;
mod matrix;
mod line;
mod triangle;
mod fragment;
mod light;
mod shaders;
mod uniforms;
mod procedural;

use framebuffer::Framebuffer;
use camera::Camera;
use obj::Obj;

use triangle::triangle;
use crate::{light::Light, matrix::{create_model_matrix, create_projection_matrix, create_view_matrix, create_viewport_matrix, multiply_matrix_vector4}, shaders::fragment_shader, uniforms::Uniforms};
use crate::procedural::{generate_uv_sphere, generate_ring};

// --- Scene entities ---
#[derive(Clone)]
enum Motion {
    Static,
    Orbit { center: Vector3, radius: f32, angular_speed: f32, phase: f32 }, // world-center orbit
    OrbitAround { parent: &'static str, radius: f32, angular_speed: f32, phase: f32 }, // orbit around entity
}

#[derive(Clone)]
enum VertexShader {
    Identity,
    DisplaceSpherical { amp: f32, freq: f32, octaves: u32, lacunarity: f32, gain: f32, time_amp: f32 },
    DisplacePlanarY  { amp: f32, freq: f32, octaves: u32, lacunarity: f32, gain: f32, time_amp: f32 },
}

#[derive(Clone)]
struct Entity {
    name: &'static str,
    translation: Vector3,
    rotation: Vector3,
    scale: f32,
    motion: Motion,
    vertices: Vec<Vector3>,
    vshader: VertexShader,
}

fn apply_vertex_shader(v: Vector3, shader: &VertexShader, time: f32) -> Vector3 {
    match shader {
        VertexShader::Identity => v,
        VertexShader::DisplaceSpherical { amp, freq, octaves, lacunarity, gain, time_amp } => {
            // Normal is radial for spheres
            let len = (v.x*v.x + v.y*v.y + v.z*v.z).sqrt().max(1e-6);
            let n = Vector3::new(v.x/len, v.y/len, v.z/len);
            // Sample FBM in object space around the surface; animate with time
            let p = Vector3::new(v.x * *freq, v.y * *freq, v.z * *freq) + Vector3::new(0.0, 0.0, time * *time_amp);
            let h = crate::procedural::fbm3(p, *octaves, *lacunarity, *gain); // ~[-1,1]
            let disp = *amp * h;
            Vector3::new(v.x + n.x * disp, v.y + n.y * disp, v.z + n.z * disp)
        }
        VertexShader::DisplacePlanarY { amp, freq, octaves, lacunarity, gain, time_amp } => {
            // For rings/planes, displace along +Y using FBM in XZ
            let p = Vector3::new(v.x * *freq, 0.0, v.z * *freq) + Vector3::new(0.0, 0.0, time * *time_amp);
            let h = crate::procedural::fbm3(p, *octaves, *lacunarity, *gain); // ~[-1,1]
            let disp = *amp * h;
            Vector3::new(v.x, v.y + disp, v.z)
        }
    }
}

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

    // Viewport una sola vez (x,y), pero mantenemos depth en NDC [-1,1] para el Z-buffer
    let screen = multiply_matrix_vector4(viewport, &ndc);
    Vector3::new(screen.x, screen.y, ndc.z)
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
    time: f32,
    resolution: Vector2,
    vshader: &VertexShader,
) {
    let light = Light::new(Vector3::new(0.0, 10.0, 0.0));
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let v_obj = apply_vertex_shader(*vertex, vshader, time);
        let transformed = transform(v_obj, translation, scale, rotation, view, projection, viewport);
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
    
    let uniforms = Uniforms {
        time,
        resolution,
    };

    // Fragment Processing Stage
    for fragment in fragments {
        let final_color = fragment.color;//fragment_shader(&fragment, &uniforms);//fragment.color;
        framebuffer.set_current_color(Color::new(final_color.x as u8, final_color.y as u8, final_color.z as u8, 255),);
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

    let projection = create_projection_matrix(PI/3.0, window_width as f32 / window_height as f32, 0.5, 100.0);
    let viewport = create_viewport_matrix(0.0, 0.0, window_width as f32, window_height as f32);

    let mut framebuffer = Framebuffer::new(window_width as u32, window_height as u32, Color::BLACK);
    framebuffer.set_background_color(Color::new(4, 12, 36, 255));

    // --- Load / build meshes ---
    // Ship from OBJ (as before)
    let ship_obj = Obj::load("nave.obj").unwrap_or_else(|_| Obj::load("sphere.obj").expect("Failed to load any mesh"));
    let ship_vertices = ship_obj.get_vertex_array();

    // Procedural planet (UV-sphere)
    let planet_vertices = generate_uv_sphere(1.2, 24, 32);

    // Procedural ring (annulus). Tip: tilt by rotating the entity (rotation.x)
    let ring_vertices = generate_ring(1.6, 2.4, 128);

    // Procedural moon (smaller UV-sphere)
    let moon_vertices = generate_uv_sphere(0.4, 16, 24);

    // --- Scene entities ---
    let mut entities: Vec<Entity> = vec![
        // The ship we will follow
        Entity {
            name: "ship",
            translation: Vector3::new(3.0, 0.0, 0.0),
            rotation: Vector3::new(0.0, 0.0, 0.0),
            scale: 1.0,
            motion: Motion::Static,
            vertices: ship_vertices.clone(),
            vshader: VertexShader::Identity,
        },
        Entity {
            name: "sun",
            translation: Vector3::new(0.0, 0.0, 0.0),
            rotation: Vector3::new(0.0, 0.0, 0.0),
            scale: 1.0,
            motion: Motion::Static,
            vertices: generate_uv_sphere(3.0, 24, 32),
            vshader: VertexShader::Identity,
        },
        
        Entity {
            name: "planet_gas",
            translation: Vector3::new(0.0, 0.0, 0.0),
            rotation: Vector3::new(0.0, 0.0, 0.0),
            scale: 1.0,
            motion: Motion::Orbit { 
                center: Vector3::new(0.0, 0.0, 0.0), radius: 20.0, angular_speed: 0.8, phase: 0.0 
            },
            vertices: planet_vertices.clone(),
            vshader: VertexShader::DisplaceSpherical { amp: 0.08, freq: 2.5, octaves: 4, lacunarity: 2.0, gain: 0.5, time_amp: 0.2 },
        },
        Entity {
            name: "planet_rocky",
            translation: Vector3::new(0.0, 0.0, 0.0),
            rotation: Vector3::new(0.0, 0.0, 0.0),
            scale: 1.0,
            motion: Motion::Orbit { 
                center: Vector3::new(0.0, 0.0, 0.0), radius: 10.0, angular_speed: 0.8, phase: 0.0 
            },
            vertices: generate_uv_sphere(0.8, 16, 24),
            vshader: VertexShader::DisplaceSpherical { amp: 0.08, freq: 2.5, octaves: 4, lacunarity: 2.0, gain: 0.5, time_amp: 0.2 },
        },
        // Planet ring (tilt a bit for a nice look)
        Entity {
            name: "planet_ring",
            translation: Vector3::new(0.0, 0.0, 0.0),
            rotation: Vector3::new(0.0, 0.0, 0.0), // tilt (radians)
            scale: 1.0,
            motion: Motion::OrbitAround {
                parent: "planet_gas",
                radius: 0.0,
                angular_speed: 0.0,
                phase: 0.0,
            },
            vertices: ring_vertices,
            vshader: VertexShader::DisplacePlanarY { amp: 0.06, freq: 6.0, octaves: 3, lacunarity: 2.0, gain: 0.55, time_amp: 0.6 },
        },
        // Moon orbiting the planet procedurally (no external model)
        Entity {
            name: "moon",
            translation: Vector3::new(0.0, 0.0, 0.0),
            rotation: Vector3::new(0.0, 0.0, 0.0),
            scale: 1.0,
            motion: Motion::OrbitAround {
                parent: "planet_rocky",
                radius: 1.5,
                angular_speed: 1.0,
                phase: 0.0,
            },
            vertices: moon_vertices,
            vshader: VertexShader::DisplaceSpherical { amp: 0.03, freq: 3.0, octaves: 3, lacunarity: 2.0, gain: 0.5, time_amp: 0.15 },
        },
    ];

    let mut camera = Camera::new(
        Vector3::new(0.0, 0.0, 50.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
    );

    let start_time = Instant::now();

    while !window.window_should_close() {
        framebuffer.clear();
        camera.process_input(&window);

        // Global time and resolution
        let time = start_time.elapsed().as_secs_f32();
        let resolution = Vector2::new(window_width as f32, window_height as f32);

        // --- Update entity motions ---
        use std::collections::HashMap;
        let index_by_name: HashMap<&'static str, usize> = entities.iter().enumerate().map(|(i,e)| (e.name, i)).collect();
        
        // Pass 1: update world-centered orbits and statics
        for i in 0..entities.len() {
            match entities[i].motion {
                Motion::Static => { /* no-op */ }
                Motion::Orbit { center, radius, angular_speed, phase } => {
                    let theta = phase + angular_speed * time;
                    entities[i].translation.x = center.x + radius * theta.cos();
                    entities[i].translation.z = center.z + radius * theta.sin();
                    entities[i].translation.y = center.y;
                    entities[i].rotation.y = -theta;
                }
                Motion::OrbitAround { .. } => { /* defer to pass 2 */ }
            }
        }
        
        // Pass 2: update children that orbit around a parent (world-axes offset around parent's position)
        for i in 0..entities.len() {
            if let Motion::OrbitAround { parent, radius, angular_speed, phase } = entities[i].motion.clone() {
                if let Some(&pi) = index_by_name.get(parent) {
                    let parent_pos = entities[pi].translation;
                    let theta = phase + angular_speed * time;

                    if radius == 0.0 {
                        // Keep centered on parent; allow spin-in-place via rotation if desired
                        entities[i].translation = parent_pos;
                        entities[i].rotation.y = -theta; // optional spin
                    } else {
                        // Orbit around parent in world axes (no coupling to parent's heading)
                        let world_offset = Vector3::new(radius * theta.cos(), 0.0, radius * theta.sin());
                        entities[i].translation = Vector3::new(
                            parent_pos.x + world_offset.x,
                            parent_pos.y + world_offset.y,
                            parent_pos.z + world_offset.z,
                        );
                        // Face tangentially to its own orbit
                        entities[i].rotation.y = -theta;
                    }
                }
            }
        }

        // --- Follow camera: lock target to ship position ---
        if let Some(ship) = entities.iter().find(|ent| ent.name == "ship") {
            camera.set_target(ship.translation);
        }

        let view = camera.get_view_matrix();

        // --- Render all entities ---
        for e in &entities {
            render(
                &mut framebuffer,
                e.translation,
                e.scale,
                e.rotation,
                &e.vertices,
                &view,
                &projection,
                &viewport,
                time,
                resolution,
                &e.vshader,
            );
        }

        framebuffer.swap_buffers(&mut window, &raylib_thread);
    }
}