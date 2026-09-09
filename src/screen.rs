use eframe::egui;

pub const TOP_WIDTH: usize = 400;
pub const TOP_HEIGHT: usize = 240;

pub const BOTTOM_WIDTH: usize = 320;
pub const BOTTOM_HEIGHT: usize = 240;

pub struct TextLine {
    pub text: String,
    pub x: f32,
    pub y: f32,
}

pub struct VirtualScreen {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u32>,
    pub text: Vec<TextLine>,
}

impl VirtualScreen {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![0x000000FF; width * height],
            text: Vec::new(),
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x >= self.width || y >= self.height {
            return;
        }

        self.pixels[y * self.width + x] = color;
    }

    pub fn draw_rect(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        color: u32,
    ) {
        let max_x = (x + width).min(self.width);
        let max_y = (y + height).min(self.height);

        for py in y..max_y {
            for px in x..max_x {
                self.pixels[py * self.width + px] = color;
            }
        }
    }

    pub fn clear(&mut self) {
        self.text.clear();

        for pixel in &mut self.pixels {
            *pixel = 0x000000FF;
        }
    }

    pub fn write_text(&mut self, text: &str) {
        let line_height = 16.0;
        let start_y = 8.0;

        for line in text.split('\n') {
            let y = start_y + self.text.len() as f32 * line_height;

            self.text.push(TextLine {
                text: line.to_string(),
                x: 8.0,
                y,
            });
        }
    }

    pub fn color_image(&self) -> egui::ColorImage {
        let pixels = self
            .pixels
            .iter()
            .map(|pixel| {
                let r = ((pixel >> 24) & 0xFF) as u8;
                let g = ((pixel >> 16) & 0xFF) as u8;
                let b = ((pixel >> 8) & 0xFF) as u8;
                let a = (pixel & 0xFF) as u8;

                egui::Color32::from_rgba_unmultiplied(r, g, b, a)
            })
            .collect();

        egui::ColorImage {
            size: [self.width, self.height],
            pixels,
            source_size: egui::vec2(self.width as f32, self.height as f32),
        }
    }

    pub fn clear_color(&mut self, color: u32) {
        self.text.clear();

        for pixel in &mut self.pixels {
            *pixel = color;
        }
    }
}