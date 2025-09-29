use raylib::prelude::*;

pub struct FrameBuffer {
    pub width: i32,
    pub height: i32,
    pub color_buffer: Image,
    background_color: Color,
    current_color: Color,
}

impl FrameBuffer {
    pub fn new(width: i32, height: i32, background_color: Color) -> Self {
        let color_buffer = Image::gen_image_color(width, height, background_color);
        FrameBuffer {
            width,
            height,
            color_buffer,
            background_color,
            current_color: Color::WHITE,
        }
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
        self.color_buffer = Image::gen_image_color(self.width, self.height, color);
    }

    pub fn clear(&mut self) {
        self.color_buffer = Image::gen_image_color(self.width, self.height, self.background_color);
    }

    pub fn set_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn set_pixel(&mut self, x: i32, y: i32) {
        if (0..self.width).contains(&x) && (0..self.height).contains(&y) {
            self.color_buffer.draw_pixel(x, y, self.current_color);
        }
    }

    pub fn set_pixel_color(&mut self, x: i32, y: i32, color: Color) {
        if (0..self.width).contains(&x) && (0..self.height).contains(&y) {
            self.color_buffer.draw_pixel(x, y, color);
        }
    }

    pub fn render_to_file(&self, file_path: &str) -> Result<(), String> {
        self.color_buffer.export_image(file_path);
        Ok(())
    }

    pub fn present_scaled(&self, d: &mut RaylibDrawHandle, dst_x: i32, dst_y: i32, scale: i32) {
        let scale = scale.max(1);

        let data = self.color_buffer.get_image_data(); // Box<[Color]>

        for y in 0..self.height {
            let row_off = (y * self.width) as usize;
            for x in 0..self.width {
                let c = data[row_off + x as usize];
                d.draw_rectangle(dst_x + x * scale, dst_y + y * scale, scale, scale, c);
            }
        }
    }
}
