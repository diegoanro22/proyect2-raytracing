use glm::vec3;
use nalgebra_glm as glm;

mod camera;
mod color;
mod intersect;
mod objects;
mod renderer;

use camera::Camera;
use color::{Material, Rgb};
use objects::{Cube, Object, Plane};
use raylib::prelude::*;

fn main() {
    let width: i32 = 640;
    let height: i32 = 360;

    let (mut rl, thread) = raylib::init()
        .size(width, height)
        .title("Raytracer - Cubo con Sombra | WASD/QE + Flechas")
        .build();

    // Cámara: arranca bien encuadrada al cubo
    // mira a y=half para ver el centro de masa del cubo
    let half = 1.1_f32; // tamaño 2.2 → half 1.1
    let mut cam = Camera::new(
        vec3(3.4, 2.0, 3.6),   // posición
        vec3(0.0, half, -5.0), // target (centro del cubo)
        vec3(0.0, 1.0, 0.0),
        60.0, // FOV más clásico
        width as f32 / height as f32,
    );

    // materiales
    let purple_dark = Material {
        diffuse: Rgb::new(55, 35, 110),
    }; // morado oscuro
    let floorc = Material {
        diffuse: Rgb::new(120, 120, 120),
    };

    // escena base: cubo centrado sobre el piso y girará solo
    let mut cube = Cube::from_center_size_rot(
        vec3(5.0, 0.2, -5.0), // center en Y=0 (lo apoyamos elevando plano)
        2.2,                  // size
        0.0,
        0.0,
        0.0, // rot inicial leve
        purple_dark,
    );

    // piso en y = -half para que el cubo “toque” el plano
    let ground_y = -half;
    let plane = Plane::new(vec3(0.0, ground_y, 0.0), vec3(0.0, 1.0, 0.0), floorc);

    while !rl.window_should_close() {
        cam.update_from_input(&rl);

        // --- rotación automática del cubo (da vueltas por defecto) ---
        let t = rl.get_time() as f32; // segundos
        let yaw = 30.0 * t; // 30° por segundo
        cube.set_rotation_euler(yaw, 0.0, 0.0);

        // construimos la lista de objetos cada frame (Clone barato)
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
