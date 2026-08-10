use bootloader_api::info::{FrameBufferInfo, PixelFormat};

const BLACK: u8 = 0x00;

#[derive(Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const WHITE: Color = Color {
        r: 255,
        g: 255,
        b: 255,
    };

    pub const GREEN: Color = Color { r: 0, g: 255, b: 0 };

    pub const BLUE: Color = Color {
        r: 80,
        g: 160,
        b: 255,
    };

    pub const RED: Color = Color {
        r: 255,
        g: 60,
        b: 60,
    };
}

pub struct Console<'a> {
    buffer: &'a mut [u8],
    info: FrameBufferInfo,
    cursor_x: usize,
    cursor_y: usize,
}

impl<'a> Console<'a> {
    pub fn new(buffer: &'a mut [u8], info: FrameBufferInfo) -> Self {
        let mut console = Self {
            buffer,
            info,
            cursor_x: 10,
            cursor_y: 10,
        };

        console.clear();
        console
    }

    pub fn clear(&mut self) {
        for byte in self.buffer.iter_mut() {
            *byte = BLACK;
        }

        self.cursor_x = 10;
        self.cursor_y = 10;
    }

    fn draw_pixel(&mut self, x: usize, y: usize, color: Color) {
        if x >= self.info.width || y >= self.info.height {
            return;
        }

        let offset = (y * self.info.stride + x) * self.info.bytes_per_pixel;

        match self.info.pixel_format {
            PixelFormat::Rgb => {
                self.buffer[offset] = color.r;
                self.buffer[offset + 1] = color.g;
                self.buffer[offset + 2] = color.b;
            }

            PixelFormat::Bgr => {
                self.buffer[offset] = color.b;
                self.buffer[offset + 1] = color.g;
                self.buffer[offset + 2] = color.r;
            }

            PixelFormat::U8 => {
                self.buffer[offset] = color.r;
            }

            _ => {}
        }
    }

    fn glyph(c: u8) -> [u8; 8] {
        match c {
            b'A' => [
                0b01110, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001, 0,
            ],

            b'I' => [
                0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b11111, 0,
            ],

            b'O' => [
                0b01110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110, 0,
            ],

            b'S' => [
                0b01111, 0b10000, 0b10000, 0b01110, 0b00001, 0b00001, 0b11110, 0,
            ],

            b'K' => [
                0b10001, 0b10010, 0b10100, 0b11000, 0b10100, 0b10010, 0b10001, 0,
            ],

            b'E' => [
                0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b11111, 0,
            ],

            b'R' => [
                0b11110, 0b10001, 0b10001, 0b11110, 0b10100, 0b10010, 0b10001, 0,
            ],

            b'N' => [
                0b10001, 0b11001, 0b10101, 0b10011, 0b10001, 0b10001, 0b10001, 0,
            ],

            b'L' => [
                0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b11111, 0,
            ],

            b'T' => [
                0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0,
            ],

            b'B' => [
                0b11110, 0b10001, 0b10001, 0b11110, 0b10001, 0b10001, 0b11110, 0,
            ],

            b'D' => [
                0b11110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b11110, 0,
            ],

            b'F' => [
                0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b10000, 0,
            ],

            b'M' => [
                0b10001, 0b11011, 0b10101, 0b10101, 0b10001, 0b10001, 0b10001, 0,
            ],

            b'[' => [
                0b11111, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b11111, 0,
            ],

            b']' => [
                0b11111, 0b00001, 0b00001, 0b00001, 0b00001, 0b00001, 0b11111, 0,
            ],

            b' ' => [0; 8],

            _ => [0; 8],
        }
    }

    fn draw_char(&mut self, x: usize, y: usize, c: u8, color: Color) {
        let glyph = Self::glyph(c);

        for (row, bits) in glyph.iter().enumerate() {
            for col in 0..5 {
                if bits & (1 << (4 - col)) != 0 {
                    self.draw_pixel(x + col, y + row, color);
                }
            }
        }
    }

    pub fn print(&mut self, text: &[u8], color: Color) {
        for &byte in text {
            if byte == b'\n' {
                self.cursor_x = 10;
                self.cursor_y += 10;
                continue;
            }

            self.draw_char(self.cursor_x, self.cursor_y, byte, color);

            self.cursor_x += 6;

            if self.cursor_x + 6 >= self.info.width {
                self.cursor_x = 10;
                self.cursor_y += 10;
            }
        }
    }

    pub fn println(&mut self, text: &[u8], color: Color) {
        self.print(text, color);

        self.cursor_x = 10;
        self.cursor_y += 10;
    }
}
