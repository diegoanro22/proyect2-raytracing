use raylib::prelude::Color;
use std::sync::Arc;

#[derive(Clone, Copy, Debug)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
impl Rgb {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
    pub fn to_raylib(self) -> Color {
        Color::new(self.r, self.g, self.b, 255)
    }

    pub fn scale(self, s: f32) -> Self {
        let f = |x: u8| ((x as f32 * s).clamp(0.0, 255.0)) as u8;
        Rgb::new(f(self.r), f(self.g), f(self.b))
    }
    pub fn add(a: Rgb, b: Rgb) -> Rgb {
        let f = |x: u8, y: u8| (x as i32 + y as i32).clamp(0, 255) as u8;
        Rgb::new(f(a.r, b.r), f(a.g, b.g), f(a.b, b.b))
    }
    pub fn mul(a: Rgb, b: Rgb) -> Rgb {
        let f = |x: u8, y: u8| ((x as u16 * y as u16) / 255) as u8;
        Rgb::new(f(a.r, b.r), f(a.g, b.g), f(a.b, b.b))
    }
    pub fn lerp(a: Rgb, b: Rgb, t: f32) -> Rgb {
        let f =
            |x: u8, y: u8| ((x as f32 * (1.0 - t) + y as f32 * t).round()).clamp(0.0, 255.0) as u8;
        Rgb::new(f(a.r, b.r), f(a.g, b.g), f(a.b, b.b))
    }
}

#[derive(Debug, Clone)]
pub enum TexSlot {
    None,
    Some(Arc<crate::texture::Texture>),
}

#[derive(Debug, Clone)]
pub struct Material {
    pub tex: TexSlot,           
    pub albedo_color: Rgb,      
    pub specular: f32,          
    pub reflectivity: f32,      
    pub transparency: f32,      
    pub ior: f32,               
    pub emission: Rgb,          
    pub emission_strength: f32,
}

impl Material {
    pub fn solid(c: Rgb) -> Self {
        Self {
            tex: TexSlot::None,
            albedo_color: c,
            specular: 0.08,
            reflectivity: 0.05,
            transparency: 0.0,
            ior: 1.0,
            emission: Rgb::new(0, 0, 0),
            emission_strength: 0.0,
        }
    }
    pub fn solid_with(c: Rgb, specular: f32, reflectivity: f32) -> Self {
        Self {
            tex: TexSlot::None,
            albedo_color: c,
            specular,
            reflectivity,
            transparency: 0.0,
            ior: 1.0,
            emission: Rgb::new(0, 0, 0),
            emission_strength: 0.0,
        }
    }
    pub fn sample_albedo(&self, uv: Option<(f32, f32)>) -> Rgb {
        match (&self.tex, uv) {
            (TexSlot::Some(tex), Some((u, v))) => {
                let t = tex.sample_repeat(u, v);
                Rgb::mul(t, self.albedo_color)
            }
            _ => self.albedo_color,
        }
    }
}
