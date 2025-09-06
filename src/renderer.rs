use glm::Vec3;
use nalgebra_glm as glm;
use raylib::prelude::*;

use crate::color::Rgb;
use crate::intersect::{Intersect, RayIntersect};
use crate::objects::Object;

const EPS: f32 = 1e-3;

fn in_shadow(point: &Vec3, normal: &Vec3, sun_dir: &Vec3, objects: &[Object]) -> bool {
    // rayo hacia la luz (direccional): -sun_dir
    let origin = point + normal * EPS;
    let dir = -sun_dir;

    for o in objects {
        let h = o.ray_intersect(&origin, &dir);
        if h.is_intersecting {
            // cualquier golpe bloquea el sol (sombras duras)
            return true;
        }
    }
    false
}

pub fn cast_ray(ro: &Vec3, rd: &Vec3, objects: &[Object]) -> Rgb {
    // z-buffer
    let mut closest = Intersect::empty();
    for o in objects {
        let hit = o.ray_intersect(ro, rd);
        if hit.is_intersecting && hit.distance < closest.distance {
            closest = hit;
        }
    }

    if !closest.is_intersecting {
        // gradiente azul (cielo)
        let top = Rgb::new(110, 160, 220); // más claro arriba
        let bottom = Rgb::new(20, 40, 90); // más oscuro abajo
        let t = 0.5 * (rd.y + 1.0);
        return Rgb::mix(bottom, top, t.clamp(0.0, 1.0));
    }

    let sun_dir = glm::normalize(&glm::vec3(-0.6, -1.1, -0.4));
    let L = -sun_dir;
    let ambient = 0.15;
    let shadowed = in_shadow(&closest.point, &closest.normal, &sun_dir, objects);
    let mut lambert = glm::dot(&closest.normal, &L).max(0.0);
    if shadowed {
        lambert = 0.0;
    }
    let shade = ambient + 1.0 * lambert;
    closest.material.diffuse.scale(shade)
}

pub fn render(
    cam: &crate::camera::Camera,
    width: i32,
    height: i32,
    objects: &[Object],
    d: &mut raylib::prelude::RaylibDrawHandle,
) {
    for y in 0..height {
        for x in 0..width {
            let dir = cam.ray_dir(x, y, width, height);
            let c = cast_ray(&cam.pos, &dir, objects).to_raylib();
            d.draw_pixel(x, y, c);
        }
    }
}
