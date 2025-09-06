use glm::{Vec3, vec3};
use nalgebra_glm as glm;

use crate::color::Material;
use crate::intersect::{Intersect, RayIntersect};

#[derive(Clone)]  
pub struct Plane {
    pub point: Vec3,  // un punto del plano
    pub normal: Vec3, // normal normalizada (mundo)
    pub material: Material,
}

impl Plane {
    pub fn new(point: Vec3, normal: Vec3, material: Material) -> Self {
        Self {
            point,
            normal: glm::normalize(&normal),
            material,
        }
    }
}

impl RayIntersect for Plane {
    fn ray_intersect(&self, ro: &Vec3, rd: &Vec3) -> Intersect {
        let denom = glm::dot(&self.normal, rd);
        // si es casi paralelo, no intersecta
        if denom.abs() < 1e-6 {
            return Intersect::empty();
        }
        let t = glm::dot(&(self.point - ro), &self.normal) / denom;
        if t <= 0.0 {
            return Intersect::empty();
        }

        let p = ro + rd * t;
        // normal siempre la del plano (constante)
        Intersect::new(p, self.normal, t, self.material)
    }
}
