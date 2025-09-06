use raylib::prelude::Color;

#[derive(Clone, Copy, Debug)]
pub struct Rgb { pub r: u8, pub g: u8, pub b: u8 }

impl Rgb {
    pub fn new(r: u8, g: u8, b: u8) -> Self { Self { r, g, b } }
    pub fn scale(self, s: f32) -> Self {
        let clamp = |x: f32| x.max(0.0).min(255.0) as u8;
        Rgb::new(clamp(self.r as f32*s), clamp(self.g as f32*s), clamp(self.b as f32*s))
    }
    pub fn mix(a: Rgb, b: Rgb, t: f32) -> Rgb {
        let lerp = |x: u8, y: u8| (x as f32*(1.0-t) + y as f32*t).round() as u8;
        Rgb::new(lerp(a.r,b.r), lerp(a.g,b.g), lerp(a.b,b.b))
    }
    pub fn to_raylib(self) -> Color { Color::new(self.r, self.g, self.b, 255) }
}

#[derive(Clone, Copy, Debug)]
pub struct Material { pub diffuse: Rgb }
