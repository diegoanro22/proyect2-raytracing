use glm::{Vec3, vec3};
use nalgebra_glm as glm;

use crate::color::Material;
use crate::intersect::{Intersect, RayIntersect};

#[derive(Clone)]
pub struct Plane {
    pub point: Vec3,     
    pub normal: Vec3,    
    pub material: Material,
    uv_axes: Option<(Vec3, Vec3, f32)>,
}

impl Plane {
    pub fn new(point: Vec3, normal: Vec3, material: Material) -> Self {
        Self {
            point,
            normal: glm::normalize(&normal),
            material,
            uv_axes: None,
        }
    }

    pub fn new_tiled(point: Vec3, normal: Vec3, material: Material, uv_scale: f32) -> Self {
        let n = glm::normalize(&normal);
        let world_up = if n.y.abs() < 0.999 { vec3(0.0, 1.0, 0.0) } else { vec3(1.0, 0.0, 0.0) };
        let u_axis = glm::normalize(&glm::cross(&world_up, &n));
        let v_axis = glm::normalize(&glm::cross(&n, &u_axis));
        Self {
            point,
            normal: n,
            material,
            uv_axes: Some((u_axis, v_axis, uv_scale)),
        }
    }
}

impl RayIntersect for Plane {
    fn ray_intersect(&self, ro: &Vec3, rd: &Vec3) -> Intersect {
        let denom = glm::dot(&self.normal, rd);
        if denom.abs() < 1e-6 { return Intersect::empty(); }
        let t = glm::dot(&(self.point - ro), &self.normal) / denom;
        if t <= 0.0 { return Intersect::empty(); }

        let p = ro + rd * t;

        let uv = if let Some((u_axis, v_axis, scale)) = self.uv_axes {
            let rel = p - self.point;
            let u = glm::dot(&rel, &u_axis) * scale;
            let v = glm::dot(&rel, &v_axis) * scale;
            Some((u, v))
        } else {
            None
        };

        Intersect::new(p, self.normal, t, self.material.clone(), uv)
    }
}
