use crate::camera::Camera;
use crate::color::{Material, Rgb};
use crate::intersect::{Intersect, RayIntersect};
use crate::objects::Object;
use nalgebra_glm as glm;
use raylib::prelude::*;

const EPS: f32 = 1e-3;
const MAX_DEPTH: i32 = 3;

#[derive(Clone, Copy, Debug)]
pub struct PointLight {
    pub pos: glm::Vec3,
    pub color: Rgb,
    pub intensity: f32,
}

fn sky_color(dir: &glm::Vec3, skybox: Option<&crate::texture::Texture>) -> Rgb {
    if let Some(tex) = skybox {
        return tex.sample_dir_equirect(dir);
    }
    let top = Rgb::new(110, 160, 220);
    let bottom = Rgb::new(20, 40, 90);
    let t = 0.5 * (dir.y + 1.0);
    Rgb::lerp(bottom, top, t.clamp(0.0, 1.0))
}

fn scene_intersect(ro: &glm::Vec3, rd: &glm::Vec3, objects: &[Object]) -> Intersect {
    let mut closest = Intersect::empty();
    for o in objects {
        let hit = o.ray_intersect(ro, rd);
        if hit.is_intersecting && hit.distance < closest.distance {
            closest = hit;
        }
    }
    closest
}

fn reflect(i: &glm::Vec3, n: &glm::Vec3) -> glm::Vec3 {
    i - 2.0 * glm::dot(i, n) * n
}

fn refract(i: &glm::Vec3, n: &glm::Vec3, etai_over_etat: f32) -> Option<glm::Vec3> {
    let cosi = (-glm::dot(i, n)).clamp(-1.0, 1.0);
    let sin2t = etai_over_etat * etai_over_etat * (1.0 - cosi * cosi);
    if sin2t > 1.0 {
        return None;
    }
    let cost = (1.0 - sin2t).sqrt();
    Some(etai_over_etat * i + (etai_over_etat * cosi - cost) * n)
}

fn fresnel_schlick(cos_theta: f32, f0: f32) -> f32 {
    f0 + (1.0 - f0) * (1.0 - cos_theta).powf(5.0)
}

fn shade(
    hit: &Intersect,
    ro: &glm::Vec3,
    rd: &glm::Vec3,
    objects: &[Object],
    depth: i32,
    skybox: Option<&crate::texture::Texture>,
    lights: &[PointLight],
) -> Rgb {
    let sun_dir = glm::normalize(&glm::vec3(-0.6, -1.0, -0.4));
    let ambient = 0.12;
    let mut lambert = glm::dot(&hit.normal, &(-sun_dir)).max(0.0);

    let shadow = {
        let origin = hit.point + hit.normal * EPS;
        let mut blocked = false;
        for o in objects {
            let h = o.ray_intersect(&origin, &(-sun_dir));
            if h.is_intersecting {
                blocked = true;
                break;
            }
        }
        if blocked { 0.0 } else { 1.0 }
    };
    lambert *= shadow;

    let base = hit.material.sample_albedo(hit.uv);
    let mut col = base.scale(ambient + lambert);

    if hit.material.specular > 0.0 {
        let view = glm::normalize(&(ro - hit.point));
        let halfv = glm::normalize(&(-sun_dir + view));
        let spec =
            glm::dot(&hit.normal, &halfv).max(0.0).powf(50.0) * hit.material.specular * shadow;
        col = Rgb::add(col, Rgb::new(255, 255, 255).scale(spec));
    }

    if hit.material.emission_strength > 0.0 {
        let e = hit.material.emission.scale(hit.material.emission_strength);
        col = Rgb::add(col, e);
    }

    for light in lights {
        let toL = light.pos - hit.point;
        let r2 = glm::dot(&toL, &toL).max(1e-6);
        let r = r2.sqrt();
        let L = toL / r;

        let origin = hit.point + hit.normal * EPS;
        let mut blocked = false;
        for o in objects {
            let h = o.ray_intersect(&origin, &L);
            if h.is_intersecting && h.distance < r - EPS {
                blocked = true;
                break;
            }
        }
        if blocked {
            continue;
        }

        let atten = light.intensity / r2;

        let ndotl = glm::dot(&hit.normal, &L).max(0.0);
        if ndotl > 0.0 {
            let diff_col = light.color.scale(ndotl * atten);
            col = Rgb::add(col, diff_col);
        }
        if hit.material.specular > 0.0 {
            let view = glm::normalize(&(ro - hit.point));
            let halfv = glm::normalize(&(L + view));
            let spec =
                glm::dot(&hit.normal, &halfv).max(0.0).powf(50.0) * hit.material.specular * atten;
            col = Rgb::add(col, Rgb::new(255, 255, 255).scale(spec));
        }
    }

    if depth >= MAX_DEPTH {
        return col;
    }

    let mut n = hit.normal;
    let mut etai_over_etat = 1.0 / hit.material.ior;
    let mut cosi = (-glm::dot(rd, &n)).clamp(-1.0, 1.0);
    let entering = glm::dot(rd, &n) < 0.0;
    if !entering {
        n = -n;
        etai_over_etat = hit.material.ior;
        cosi = (-glm::dot(rd, &n)).clamp(-1.0, 1.0);
    }

    let f0_base = ((1.0 - hit.material.ior) / (1.0 + hit.material.ior)).powi(2);
    let fres = fresnel_schlick(cosi, f0_base).clamp(0.0, 1.0);

    if hit.material.reflectivity > 0.0 || (hit.material.transparency > 0.0 && fres > 0.0) {
        let rdir = glm::normalize(&reflect(rd, &hit.normal));
        let rorig = hit.point + hit.normal * EPS;
        let rcol = cast_ray(&rorig, &rdir, objects, depth + 1, skybox, lights);
        let mixf = (hit.material.reflectivity + fres).clamp(0.0, 1.0);
        col = Rgb::lerp(col, rcol, mixf);
    }

    if hit.material.transparency > 0.0 {
        if let Some(tdir) = refract(rd, &n, etai_over_etat) {
            let torig = hit.point - n * EPS; // empuja hacia adentro
            let tcol = cast_ray(
                &torig,
                &glm::normalize(&tdir),
                objects,
                depth + 1,
                skybox,
                lights,
            );
            let atten = 0.85;
            let tcol_att = tcol.scale(atten);
            let mixf = (hit.material.transparency * (1.0 - fres)).clamp(0.0, 1.0);
            col = Rgb::lerp(col, tcol_att, mixf);
        }
    }

    col
}

fn cast_ray(
    ro: &glm::Vec3,
    rd: &glm::Vec3,
    objects: &[Object],
    depth: i32,
    skybox: Option<&crate::texture::Texture>,
    lights: &[PointLight],
) -> Rgb {
    let hit = scene_intersect(ro, rd, objects);
    if !hit.is_intersecting {
        return sky_color(rd, skybox);
    }
    shade(&hit, ro, rd, objects, depth, skybox, lights)
}

pub fn render(
    cam: &Camera,
    width: i32,
    height: i32,
    objects: &[Object],
    d: &mut raylib::prelude::RaylibDrawHandle,
    skybox: Option<&crate::texture::Texture>,
    lights: &[PointLight],
) {
    for y in 0..height {
        for x in 0..width {
            let dir = cam.ray_dir(x, y, width, height);
            let c = cast_ray(&cam.pos, &dir, objects, 0, skybox, lights).to_raylib();
            d.draw_pixel(x, y, c);
        }
    }
}

pub fn render_to_fb(
    cam: &Camera,
    fb: &mut crate::framebuffer::FrameBuffer,
    objects: &[Object],
    skybox: Option<&crate::texture::Texture>,
    lights: &[PointLight],
) {
    let w = fb.width;
    let h = fb.height;

    fb.clear();

    for y in 0..h {
        for x in 0..w {
            let dir = cam.ray_dir(x, y, w, h);
            let rgb: Rgb = cast_ray(&cam.pos, &dir, objects, 0, skybox, lights);
            fb.set_pixel(x, y);
            fb.set_color(rgb.to_raylib()); 
        }
    }
}
