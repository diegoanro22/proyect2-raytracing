use glm::{Mat3, Vec3, vec3};
use nalgebra_glm as glm;

use crate::color::Material;
use crate::intersect::{Intersect, RayIntersect};

#[derive(Clone)]
pub struct Cube {
    pub center: Vec3,
    pub half: Vec3,
    pub rot: Mat3,     // mundo <- local
    pub rot_inv: Mat3, // local <- mundo
    pub material: Material,
}

impl Cube {
    pub fn from_center_size_rot(
        center: Vec3,
        size: f32,
        yaw_deg: f32,
        pitch_deg: f32,
        roll_deg: f32,
        material: Material,
    ) -> Self {
        let half = vec3(size * 0.5, size * 0.5, size * 0.5);
        let q_yaw = glm::quat_angle_axis(yaw_deg.to_radians(), &vec3(0.0, 1.0, 0.0));
        let q_pitch = glm::quat_angle_axis(pitch_deg.to_radians(), &vec3(1.0, 0.0, 0.0));
        let q_roll = glm::quat_angle_axis(roll_deg.to_radians(), &vec3(0.0, 0.0, 1.0));
        let q = q_yaw * q_pitch * q_roll;
        let rot: Mat3 = glm::quat_to_mat3(&q);
        let rot_inv: Mat3 = rot.transpose();
        Self {
            center,
            half,
            rot,
            rot_inv,
            material,
        }
    }
    pub fn set_rotation_euler(&mut self, yaw_deg: f32, pitch_deg: f32, roll_deg: f32) {
        let q_yaw = glm::quat_angle_axis(yaw_deg.to_radians(), &vec3(0.0, 1.0, 0.0));
        let q_pitch = glm::quat_angle_axis(pitch_deg.to_radians(), &vec3(1.0, 0.0, 0.0));
        let q_roll = glm::quat_angle_axis(roll_deg.to_radians(), &vec3(0.0, 0.0, 1.0));
        let q = q_yaw * q_pitch * q_roll;
        self.rot = glm::quat_to_mat3(&q);
        self.rot_inv = self.rot.transpose();
    }
}

impl RayIntersect for Cube {
    fn ray_intersect(&self, ro_w: &Vec3, rd_w: &Vec3) -> Intersect {
        // mundo -> local
        let ro = self.rot_inv * (ro_w - self.center);
        let rd = self.rot_inv * *rd_w;

        let min = -self.half;
        let max = self.half;

        let inv = vec3(
            if rd.x != 0.0 {
                1.0 / rd.x
            } else {
                f32::INFINITY
            },
            if rd.y != 0.0 {
                1.0 / rd.y
            } else {
                f32::INFINITY
            },
            if rd.z != 0.0 {
                1.0 / rd.z
            } else {
                f32::INFINITY
            },
        );

        let mut t1 = (min.x - ro.x) * inv.x;
        let mut t2 = (max.x - ro.x) * inv.x;
        let mut t3 = (min.y - ro.y) * inv.y;
        let mut t4 = (max.y - ro.y) * inv.y;
        let mut t5 = (min.z - ro.z) * inv.z;
        let mut t6 = (max.z - ro.z) * inv.z;

        if t1 > t2 {
            std::mem::swap(&mut t1, &mut t2);
        }
        if t3 > t4 {
            std::mem::swap(&mut t3, &mut t4);
        }
        if t5 > t6 {
            std::mem::swap(&mut t5, &mut t6);
        }

        let tmin = t1.max(t3).max(t5);
        let tmax = t2.min(t4).min(t6);
        if tmax < 0.0 || tmin > tmax {
            return Intersect::empty();
        }

        let t_hit = if tmin > 0.0 { tmin } else { tmax };
        if t_hit <= 0.0 {
            return Intersect::empty();
        }

        // punto & normal locales
        let p_local = ro + rd * t_hit;
        let eps = 1e-3;
        let mut n_local = vec3(0.0, 0.0, 0.0);
        if (p_local.x - min.x).abs() < eps {
            n_local = vec3(-1.0, 0.0, 0.0);
        } else if (p_local.x - max.x).abs() < eps {
            n_local = vec3(1.0, 0.0, 0.0);
        } else if (p_local.y - min.y).abs() < eps {
            n_local = vec3(0.0, -1.0, 0.0);
        } else if (p_local.y - max.y).abs() < eps {
            n_local = vec3(0.0, 1.0, 0.0);
        } else if (p_local.z - min.z).abs() < eps {
            n_local = vec3(0.0, 0.0, -1.0);
        } else if (p_local.z - max.z).abs() < eps {
            n_local = vec3(0.0, 0.0, 1.0);
        }

        // local -> mundo
        let p_world = self.center + self.rot * p_local;
        let n_world = self.rot * n_local;

        Intersect::new(p_world, n_world, t_hit, self.material)
    }
}
