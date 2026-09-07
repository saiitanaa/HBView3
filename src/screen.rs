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
    pub text: Vec<TextLine>,
}

impl VirtualScreen {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            text: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.text.clear();
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

    pub fn draw(&self, painter: &egui::Painter, rect: egui::Rect, scale: f32) {
        painter.rect_filled(
            rect,
            0.0,
            egui::Color32::BLACK,
        );

        for line in &self.text {
            painter.text(
                rect.min + egui::vec2(line.x, line.y) * scale,
                egui::Align2::LEFT_TOP,
                &line.text,
                egui::FontId::monospace(13.0 * scale),
                egui::Color32::WHITE,
            );
        }
    }
}