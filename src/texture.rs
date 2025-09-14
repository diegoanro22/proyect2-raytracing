use crate::color::Rgb;
use raylib::prelude::*;

#[derive(Debug, Clone)]
pub struct Texture {
    pub w: i32,
    pub h: i32,
    pub pixels: Vec<Rgb>,
}

impl Texture {
    pub fn from_file(path: &str) -> Self {
        let img = Image::load_image(path).expect("No pude cargar la textura");
        let w = img.width();
        let h = img.height();

        let data = img.get_image_data(); // Box<[Color]>
        let mut pixels = Vec::with_capacity((w * h) as usize);
        for c in data.iter() {
            // <- iter por referencia
            pixels.push(Rgb::new(c.r, c.g, c.b));
        }

        Self { w, h, pixels }
    }

    pub fn sample(&self, mut u: f32, mut v: f32) -> Rgb {
        u = u.fract();
        if u < 0.0 {
            u += 1.0;
        }
        v = v.fract();
        if v < 0.0 {
            v += 1.0;
        }
        let x = (u * self.w as f32) as i32;
        let y = ((1.0 - v) * self.h as f32) as i32;
        let xi = x.clamp(0, self.w - 1);
        let yi = y.clamp(0, self.h - 1);
        self.pixels[(yi * self.w + xi) as usize]
    }
}
