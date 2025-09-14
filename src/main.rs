use glm::vec3;
use nalgebra_glm as glm;

mod camera;
mod color;
mod intersect;
mod objects;
mod renderer;
mod texture; // <--- NUEVO

use camera::Camera;
use color::{Material, Rgb};
use objects::{Cube, Object, Plane};
use raylib::prelude::*;
use std::sync::Arc;

fn main() {
    let width: i32 = 1000;
    let height: i32 = 600;

    let (mut rl, thread) = raylib::init()
        .size(width, height)
        .title("Raytracer - Cubo con Sombra | WASD/QE + Flechas")
        .build();

    // Cámara
    let half = 1.1_f32;
    let mut cam = Camera::new(
        vec3(3.4, 2.0, 3.6),
        vec3(0.0, half, -5.0),
        vec3(0.0, 1.0, 0.0),
        60.0,
        width as f32 / height as f32,
    );

    // --- materiales ---
    let tex = Arc::new(texture::Texture::from_file("assets/texture.png"));
    let mat_cubo = Material::textured(tex); // SOLO textura (sin color)
    let mat_piso = Material::solid(Rgb::new(120, 120, 120)); // piso gris

    // escena: cubo texturizado + piso
    let mut cube = Cube::from_center_size_rot(vec3(5.0, 0.2, -5.0), 2.2, 0.0, 0.0, 0.0, mat_cubo);

    let ground_y = -half;
    let plane = Plane::new(vec3(0.0, ground_y, 0.0), vec3(0.0, 1.0, 0.0), mat_piso);

    while !rl.window_should_close() {
        cam.update_from_input(&rl);

        // rotación automática
        let t = rl.get_time() as f32;
        let yaw = 30.0 * t;
        cube.set_rotation_euler(yaw, 0.0, 0.0);

        let objs = [Object::Plane(plane.clone()), Object::Cube(cube.clone())];

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        renderer::render(&cam, width, height, &objs, &mut d);

        d.draw_text(
            "WASD/QE moverse | Flechas rotar | Shift=rapido",
            10,
            10,
            16,
            Color::WHITE,
        );
        d.draw_fps(10, height - 24);
    }
}
