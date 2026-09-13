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
}