use glm::{Vec3, vec3};
use nalgebra_glm as glm;

pub struct Camera {
    pub pos: Vec3,
    pub yaw: f32,   // en radianes
    pub pitch: f32, // en radianes
    pub fov_tan: f32,
    pub aspect: f32,
    // bases derivadas
    pub forward: Vec3,
    pub right: Vec3,
    pub up_cam: Vec3,
}

impl Camera {
    pub fn new(pos: Vec3, target: Vec3, up: Vec3, fov_deg: f32, aspect: f32) -> Self {
        let forward = glm::normalize(&(target - pos));
        let yaw = forward.z.atan2(forward.x) + std::f32::consts::PI; // aproximación
        let pitch = (-forward.y).asin();

        let mut cam = Self {
            pos,
            yaw,
            pitch,
            fov_tan: (fov_deg.to_radians() * 0.5).tan(),
            aspect,
            forward,
            right: vec3(1.0, 0.0, 0.0),
            up_cam: vec3(0.0, 1.0, 0.0),
        };
        cam.rebuild_basis();
        cam
    }

    fn rebuild_basis(&mut self) {
        // a partir de yaw/pitch, estilo FPS
        let cy = self.yaw.cos();
        let sy = self.yaw.sin();
        let cp = self.pitch.cos();
        let sp = self.pitch.sin();

        self.forward = glm::normalize(&vec3(cy * cp, sp, -sy * cp));
        self.right = glm::normalize(&glm::cross(&self.forward, &vec3(0.0, 1.0, 0.0)));
        self.up_cam = glm::normalize(&glm::cross(&self.right, &self.forward));
    }

    pub fn set_aspect(&mut self, aspect: f32) {
        self.aspect = aspect;
    }

    pub fn ray_dir(&self, px: i32, py: i32, w: i32, h: i32) -> Vec3 {
        let u = (((px as f32 + 0.5) / w as f32) * 2.0 - 1.0) * self.aspect * self.fov_tan;
        let v = (1.0 - ((py as f32 + 0.5) / h as f32) * 2.0) * self.fov_tan;
        glm::normalize(&(self.forward + self.right * u + self.up_cam * v))
    }

    pub fn update_from_input(&mut self, rl: &raylib::RaylibHandle) {
        use raylib::consts::KeyboardKey::*;
        // rotación con flechas
        let rot_speed = 1.2_f32.to_radians(); // rad/seg aprox (es por frame, simple)
        if rl.is_key_down(KEY_RIGHT) {
            self.yaw -= rot_speed;
        }
        if rl.is_key_down(KEY_LEFT) {
            self.yaw += rot_speed;
        }
        if rl.is_key_down(KEY_UP) {
            self.pitch += rot_speed;
        }
        if rl.is_key_down(KEY_DOWN) {
            self.pitch -= rot_speed;
        }
        self.pitch = self.pitch.clamp(-1.3, 1.3);
        self.rebuild_basis();

        // movimiento con WASD + Q/E
        let base = if rl.is_key_down(KEY_LEFT_SHIFT) {
            0.15
        } else {
            0.06
        };
        if rl.is_key_down(KEY_W) {
            self.pos += self.forward * base;
        }
        if rl.is_key_down(KEY_S) {
            self.pos -= self.forward * base;
        }
        if rl.is_key_down(KEY_A) {
            self.pos -= self.right * base;
        }
        if rl.is_key_down(KEY_D) {
            self.pos += self.right * base;
        }
        if rl.is_key_down(KEY_Q) {
            self.pos -= self.up_cam * base;
        }
        if rl.is_key_down(KEY_E) {
            self.pos += self.up_cam * base;
        }
    }
}
