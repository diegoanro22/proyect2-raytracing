use glm::vec3;
use nalgebra_glm as glm;
use raylib::prelude::*;
use std::sync::Arc;

mod camera;
mod color;
mod framebuffer;
mod intersect;
mod objects;
mod renderer;
mod texture;

use camera::Camera;
use color::{Material, Rgb, TexSlot};
use framebuffer::FrameBuffer;
use objects::{Cube, Object, Plane};

fn main() {
    let width: i32 = 960;
    let height: i32 = 540;

    let (_rl, _thread) = raylib::init()
        .size(width, height)
        .title("Diorama Raytracer — Casa ladrillo (plataforma grande, árbol, charco)")
        .build();

    // Cámara
    let mut cam = Camera::new(
        vec3(12.0, 1.8, 0.8),
        vec3(15.0, 3.6, 0.0),
        vec3(0.0, 1.0, 10.0),
        60.0,
        width as f32 / height as f32,
    );

    // Texturas
    let tex_brick = Arc::new(texture::Texture::from_file("assets/texture.png"));
    let tex_ground = Arc::new(texture::Texture::from_file("assets/ground.png"));
    let tex_glowstone = Arc::new(texture::Texture::from_file("assets/glowstone.png"));
    let skybox = texture::Texture::from_file_maybe("assets/skybox.png");
    let tex_bark = Arc::new(texture::Texture::from_file("assets/bark.png"));
    let tex_leaves = Arc::new(texture::Texture::from_file("assets/leaves.png"));

    // Materiales
    let mat_brick = color::Material {
        tex: TexSlot::Some(tex_brick.clone()),
        albedo_color: Rgb::new(200, 170, 120),
        specular: 0.12,
        reflectivity: 0.08,
        transparency: 0.0,
        ior: 1.0,
        emission: Rgb::new(0, 0, 0),
        emission_strength: 0.0,
    };
    let mat_ground = color::Material {
        tex: TexSlot::Some(tex_ground.clone()),
        albedo_color: Rgb::new(255, 255, 255),
        specular: 0.05,
        reflectivity: 0.02,
        transparency: 0.0,
        ior: 1.0,
        emission: Rgb::new(0, 0, 0),
        emission_strength: 0.0,
    };
    let mat_glowstone = color::Material {
        tex: TexSlot::Some(tex_glowstone.clone()),
        albedo_color: Rgb::new(255, 255, 200),
        specular: 0.30,
        reflectivity: 0.10,
        transparency: 0.0,
        ior: 1.0,
        emission: Rgb::new(255, 240, 200),
        emission_strength: 0.5,
    };
    let mat_metal = Material::solid_with(Rgb::new(180, 180, 190), 0.75, 0.65);
    let mat_vidrio = color::Material {
        tex: TexSlot::None,
        albedo_color: Rgb::new(200, 220, 235),
        specular: 0.25,
        reflectivity: 0.08,
        transparency: 0.92,
        ior: 1.52,
        emission: Rgb::new(0, 0, 0),
        emission_strength: 0.0,
    };
    let mat_agua = color::Material {
        tex: TexSlot::None,
        albedo_color: Rgb::new(110, 140, 180),
        specular: 0.20,
        reflectivity: 0.08,
        transparency: 0.80,
        ior: 1.33,
        emission: Rgb::new(0, 0, 0),
        emission_strength: 0.0,
    };
    let mat_tronco = color::Material {
        tex: TexSlot::Some(tex_bark.clone()),
        albedo_color: Rgb::new(255, 255, 255), 
        specular: 0.06,
        reflectivity: 0.03,
        transparency: 0.0,
        ior: 1.0,
        emission: Rgb::new(0, 0, 0),
        emission_strength: 0.0,
    };

    let mat_hojas = color::Material {
        tex: TexSlot::Some(tex_leaves.clone()),
        albedo_color: Rgb::new(255, 255, 255), // idem
        specular: 0.04,
        reflectivity: 0.02,
        transparency: 0.0, 
        ior: 1.0,
        emission: Rgb::new(0, 0, 0),
        emission_strength: 0.0,
    };

    // ===== Framebuffer =====
    let fb_w = width / 2;
    let fb_h = height / 2;
    let mut fb = FrameBuffer::new(fb_w, fb_h, Color::BLACK);
    cam.set_aspect(fb_w as f32 / fb_h as f32);

    // ======= ESCENA =======
    let mut objs: Vec<Object> = vec![];

    // Parámetros “voxel”
    let b: f32 = 0.4; // tamaño de cubo
    let nx: i32 = 10; // ancho casa
    let nz: i32 = 8; // profundidad casa
    let ny: i32 = 5; // altura paredes

    // Referencias
    let world_floor_y = -0.6; // piso del mundo
    let base_y = world_floor_y + b * 1.5; // casa subida 1 bloque vs mundo
    let house_center = vec3(0.0, 0.0, -5.0);

    let halfx = (nx as f32 - 1.0) * 0.5;
    let halfz = (nz as f32 - 1.0) * 0.5;
    let pos = |ix: i32, iy: i32, iz: i32| -> glm::Vec3 {
        house_center
            + vec3(
                (ix as f32 - halfx) * b,
                base_y + (iy as f32) * b,
                (iz as f32 - halfz) * b,
            )
    };

    // --- Plataforma grande ---
    let pad_plat = 6; // <- MÁS GRANDE
    let nxp = nx + pad_plat * 2;
    let nzp = nz + pad_plat * 2;
    let plat_center = house_center;
    let plat_y = world_floor_y + b * 0.5; // toca el mundo
    let halfx_p = (nxp as f32 - 1.0) * 0.5;
    let halfz_p = (nzp as f32 - 1.0) * 0.5;

    for ix in 0..nxp {
        for iz in 0..nzp {
            let p =
                plat_center + vec3((ix as f32 - halfx_p) * b, plat_y, (iz as f32 - halfz_p) * b);
            let tile = Cube::from_center_size_rot(p, b, 0.0, 0.0, 0.0, mat_ground.clone());
            objs.push(Object::Cube(tile));
        }
    }

    // --- Casa (aperturas) ---
    let door_w_blocks = 2;
    let door_h_blocks = 3;
    let door_ix_center = nx / 2;
    let front_z = nz - 1;

    let win_w_blocks = 2;
    let win_h_blocks = 2;
    let win_y_center = 3;

    // Paredes de ladrillo con huecos
    for iy in 0..ny {
        for ix in 0..nx {
            for iz in 0..nz {
                let is_front = iz == front_z;
                let is_back = iz == 0;
                let is_left = ix == 0;
                let is_right = ix == nx - 1;
                let on_perimeter = is_front || is_back || is_left || is_right;
                if !on_perimeter {
                    continue;
                }

                // puerta
                if is_front {
                    let in_door_x = (ix >= door_ix_center - (door_w_blocks / 2))
                        && (ix <= door_ix_center + (door_w_blocks / 2) - 1);
                    let in_door_y = iy < door_h_blocks;
                    if in_door_x && in_door_y {
                        continue;
                    }
                }

                // ventanas en izq/der
                let win_z0 = nz / 2 - win_w_blocks / 2;
                let win_z1 = win_z0 + win_w_blocks - 1;
                let win_y0 = win_y_center - win_h_blocks / 2;
                let win_y1 = win_y0 + win_h_blocks - 1;
                if is_left || is_right {
                    if iy >= win_y0 && iy <= win_y1 && iz >= win_z0 && iz <= win_z1 {
                        continue;
                    }
                }

                let c = Cube::from_center_size_rot(
                    pos(ix, iy, iz),
                    b,
                    0.0,
                    0.0,
                    0.0,
                    mat_brick.clone(),
                );
                objs.push(Object::Cube(c));
            }
        }
    }

    // Dintel
    for ix in (door_ix_center - (door_w_blocks / 2))..=(door_ix_center + (door_w_blocks / 2) - 1) {
        let iy = door_h_blocks;
        let c =
            Cube::from_center_size_rot(pos(ix, iy, front_z), b, 0.0, 0.0, 0.0, mat_brick.clone());
        objs.push(Object::Cube(c));
    }

    // Ventanas 
    for dz in 0..win_w_blocks {
        for dy in 0..win_h_blocks {
            let iz = (nz / 2 - win_w_blocks / 2) + dz;
            let iy = (win_y_center - win_h_blocks / 2) + dy;

            let c_l = Cube::from_center_size_rot(
                pos(0, iy, iz),
                b * 0.9,
                0.0,
                0.0,
                0.0,
                mat_vidrio.clone(),
            );
            let c_r = Cube::from_center_size_rot(
                pos(nx - 1, iy, iz),
                b * 0.9,
                0.0,
                0.0,
                0.0,
                mat_vidrio.clone(),
            );
            objs.push(Object::Cube(c_l));
            objs.push(Object::Cube(c_r));
        }
    }

    // Techo con glowstone + luces
    let mut lights: Vec<renderer::PointLight> = vec![];
    let roof_y = ny;
    for ix in 0..nx {
        for iz in 0..nz {
            let p = pos(ix, roof_y, iz);
            let c = Cube::from_center_size_rot(p, b, 0.0, 0.0, 0.0, mat_glowstone.clone());
            objs.push(Object::Cube(c));
            lights.push(renderer::PointLight {
                pos: p,
                color: Rgb::new(255, 240, 200),
                intensity: 6.0,
            });
        }
    }

    // --- Árbol sobre la plataforma ---
    let tree_base_x = plat_center.x - (halfx_p - 2.0) * b;
    let tree_base_z = plat_center.z + (halfz_p - 2.5) * b;
    let tree_base_y = plat_y + b; // que arranque sobre la plataforma

    // Tronco (columna)
    let trunk_h = 4;
    for i in 0..trunk_h {
        let p = vec3(tree_base_x, tree_base_y + (i as f32) * b, tree_base_z);
        let trunk_block = Cube::from_center_size_rot(p, b, 0.0, 0.0, 0.0, mat_tronco.clone());
        objs.push(Object::Cube(trunk_block));
    }

    // Copa 
    let crown_center_y = tree_base_y + (trunk_h as f32) * b;
    for dx in -1..=1 {
        for dy in -1..=1 {
            for dz in -1..=1 {
                // opcional: hacer la copa más redondeada quitando esquinas
                let p = vec3(
                    tree_base_x + (dx as f32) * b,
                    crown_center_y + (dy as f32) * b,
                    tree_base_z + (dz as f32) * b,
                );
                let leaf = Cube::from_center_size_rot(p, b, 0.0, 0.0, 0.0, mat_hojas.clone());
                objs.push(Object::Cube(leaf));
            }
        }
    }

    // --- Charco/agua junto al árbol (2x2 bloques) ---
    for ax in 0..2 {
        for az in 0..2 {
            let p = vec3(
                tree_base_x + (1 + ax) as f32 * b, // a la par del árbol
                plat_y,                            // sobre la plataforma
                tree_base_z + az as f32 * b,
            );
            let water = Cube::from_center_size_rot(p, b, 0.0, 0.0, 0.0, mat_agua.clone());
            objs.push(Object::Cube(water));
        }
    }

    // ===== Render y guardar snapshot =====
    renderer::render_to_fb(&cam, &mut fb, &objs, skybox.as_ref(), &lights);
    fb.render_to_file("output2.png").unwrap();
}
