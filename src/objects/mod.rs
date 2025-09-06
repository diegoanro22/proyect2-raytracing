use glm::Vec3;
use nalgebra_glm as glm;

use crate::intersect::{Intersect, RayIntersect};

pub mod cube;
pub use cube::Cube;

pub mod plane; // <--- NUEVO
pub use plane::Plane; // <--- NUEVO

pub enum Object {
    Cube(Cube),
    Plane(Plane), // <--- NUEVO
}

impl RayIntersect for Object {
    fn ray_intersect(&self, ro: &Vec3, rd: &Vec3) -> Intersect {
        match self {
            Object::Cube(c) => c.ray_intersect(ro, rd),
            Object::Plane(p) => p.ray_intersect(ro, rd), // <--- NUEVO
        }
    }
}
