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

    pub fn draw_line(
        &mut self,
        x1: usize,
        y1: usize,
        x2: usize,
        y2: usize,
        color: u32,
    ) {
        let mut x1 = x1 as isize;
        let mut y1 = y1 as isize;
        let x2 = x2 as isize;
        let y2 = y2 as isize;
        
        let dx = (x2 - x1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let dy = -(y2 - y1).abs();
        let sy = if y1 < y2 { 1 } else { -1 };
        
        let mut error = dx + dy;
        loop {
            if x1 >= 0 && y1 >= 0 {
                self.draw_pixel(
                    x1 as usize,
                    y1 as usize,
                    color,
                );
            }

            if x1 == x2 && y1 == y2 {
                break;
            }

            let error2 = 2 * error;
            if error2 >= dy {
                error += dy;
                x1 += sx;
            }

            if error2 <= dx {
                error += dx;
                y1 += sy;
            }
        }
    }
    
    pub fn draw_circle(
        &mut self,
        center_x: usize,
        center_y: usize,
        radius: usize,
        color: u32,
    ) {
        let center_x = center_x as isize;
        let center_y = center_y as isize;
        let radius = radius as isize;

        let mut x = radius;
        let mut y = 0isize;
        let mut error = 1 - radius;

        while x >= y {
            let points = [
                (center_x + x, center_y + y),
                (center_x + y, center_y + x),
                (center_x - y, center_y + x),
                (center_x - x, center_y + y),
                (center_x - x, center_y - y),
                (center_x - y, center_y - x),
                (center_x + y, center_y - x),
                (center_x + x, center_y - y),
            ];

            for (x, y) in points {
                if x >= 0 && y >= 0 {
                    self.draw_pixel(
                        x as usize,
                        y as usize,
                        color,
                    );
                }
            }

            y += 1;

            if error < 0 {
                error += 2 * y + 1;
            } else {
                x -= 1;
                error += 2 * (y - x) + 1;
            }
        }
    }

    pub fn draw_ellipse (
        &mut self,
        center_x: usize,
        center_y: usize,
        radius_x: usize,
        radius_y: usize,
        color: u32,
    ) {
        let center_x = center_x as isize;
        let center_y = center_y as isize;
        let radius_x = radius_x as isize;
        let radius_y = radius_y as isize;

        if radius_x == 0 || radius_y == 0 {
            return;
        }

        let radius_x_squared = radius_x * radius_x;
        let radius_y_squared = radius_y * radius_y;
        let mut x = 0isize;
        let mut y = radius_y;
        let mut dx = 2 * radius_y_squared * x;
        let mut dy = 2 * radius_x_squared * y;
        let mut decision = radius_y_squared - radius_x_squared * radius_y + radius_x_squared / 4;

        while dx < dy {
            self.draw_pixel((center_x + x) as usize, (center_y + y) as usize, color);
            self.draw_pixel((center_x - x) as usize, (center_y - y) as usize, color);
            self.draw_pixel((center_x - x) as usize, (center_y - y) as usize, color);
            self.draw_pixel((center_x + x) as usize, (center_y - y) as usize, color);
            self.draw_pixel((center_x - x) as usize, (center_y + y) as usize, color);

            x += 1;
            dx += 2 * radius_y_squared;
            if decision < 0 {
                decision += dx + radius_y_squared;
            } else {
                y -= 1;
                dy -= 2 * radius_x_squared;
                decision += dx - dy + radius_y_squared;
            }
        }

        decision = radius_y_squared * (x * x + x) + radius_x_squared * (y * y - 2 * y + 1) - radius_x_squared * radius_y_squared;
        while y >= 0 {
            self.draw_pixel((center_x + x) as usize, (center_y + y) as usize, color);
            self.draw_pixel((center_x + x) as usize, (center_y + y) as usize, color);
            self.draw_pixel((center_x - x) as usize, (center_y + y) as usize, color);
            self.draw_pixel((center_x - x) as usize, (center_y - y) as usize, color);
            self.draw_pixel((center_x + x) as usize, (center_y - y) as usize, color);
            
            y -= 1;
            dy -= 2 * radius_x_squared;
            if decision > 0 {
                decision += radius_x_squared - dy;
            } else {
                x += 1;
                dx +=2 * radius_y_squared;
                decision += dx - dy + radius_x_squared;
            }
        }
    }

    pub fn draw_triangle (
        &mut self,
        x1: usize,
        y1: usize,
        x2: usize,
        y2: usize,
        x3: usize,
        y3: usize,
        color: u32,
    ) {
        self.draw_line(x1, y1, x2, y2, color);
        self.draw_line(x2, y2, x3, y3, color);
        self.draw_line(x3, y3, x1, y1, color);
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