use serde::Serialize;

pub struct TextLine {
    pub text: String,
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Serialize)]
pub struct Pixel {
    pub x: usize,
    pub y: usize,
    pub color: u32,
}

#[derive(Debug, Serialize)]
pub struct Rect {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
    pub color: u32,
}

pub struct VirtualScreen {
    pub width: usize,
    pub height: usize,
    pub text: Vec<TextLine>,
    pub pixels: Vec<Pixel>,
    pub rectangles: Vec<Rect>,
    pub background_color: u32,
}

impl VirtualScreen {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            text: Vec::new(),
            pixels: Vec::new(),
            rectangles: Vec::new(),
            background_color: 0x000000,
        }
    }

    pub fn clear(&mut self, color: u32) {
        self.background_color = color;
        self.pixels.clear();
        self.rectangles.clear();
        self.text.clear();
    }

    pub fn draw_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x >= self.width || y >= self.height {
            return;
        }

        self.pixels.push(Pixel { x, y, color });
    }

    pub fn draw_rect(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        color: u32,
    ) {
        if x >= self.width || y >= self.height {
            return;
        }

        self.rectangles.push(Rect {
            x,
            y,
            width,
            height,
            color,
        });
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

    pub fn pixels(&self) -> &[Pixel] {
        &self.pixels
    }

    pub fn rectangles(&self) -> &[Rect] {
        &self.rectangles
    }

    pub fn background_color(&self) -> u32 {
        self.background_color
    }
}