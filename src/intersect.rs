use crate::color::Material;
use glm::{Vec3, vec3};
use nalgebra_glm as glm;

#[derive(Clone, Debug)]
pub struct Intersect {
    pub distance: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub is_intersecting: bool,
    pub material: Material,
    pub uv: Option<(f32, f32)>,
}

impl Intersect {
    pub fn empty() -> Self {
        Self {
            distance: f32::INFINITY,
            point: vec3(0.0, 0.0, 0.0),
            normal: vec3(0.0, 0.0, 0.0),
            is_intersecting: false,
            material: Material::solid(crate::color::Rgb::new(0, 0, 0)),
            uv: None,
        }
    }
    pub fn new(
        point: Vec3,
        normal: Vec3,
        distance: f32,
        material: Material,
        uv: Option<(f32, f32)>,
    ) -> Self {
        Self {
            distance,
            point,
            normal: glm::normalize(&normal),
            is_intersecting: true,
            material,
            uv,
        }
    }
}

pub trait RayIntersect {
    fn ray_intersect(&self, ro: &Vec3, rd: &Vec3) -> Intersect;
}
