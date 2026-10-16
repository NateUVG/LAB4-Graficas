#[derive(Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    pub fn as_u32(self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | self.b as u32
    }
}

pub struct FrameBuffer {
    width: usize,
    height: usize,
    buffer: Vec<u32>,
    background: Color,
    current: Color,
}

impl FrameBuffer {
    pub fn new(width: usize, height: usize, background: Color) -> Self {
        FrameBuffer {
            width,
            height,
            buffer: vec![background.as_u32(); width * height],
            background,
            current: Color::new(255, 255, 255),
        }
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background = color;
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current = color;
    }

    pub fn clear(&mut self) {
        self.buffer.fill(self.background.as_u32());
    }

    pub fn point(&mut self, x: i32, y: i32) {
        if x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height {
            self.buffer[y as usize * self.width + x as usize] = self.current.as_u32();
        }
    }

    pub fn buffer(&self) -> &[u32] {
        &self.buffer
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }
}
