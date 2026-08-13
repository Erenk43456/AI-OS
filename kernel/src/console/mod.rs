use bootloader_api::info::{FrameBufferInfo, PixelFormat};

// ============================================================
// COLOR
// ============================================================

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
        r: 0,
        g: 100,
        b: 255,
    };

    pub const RED: Color = Color { r: 255, g: 0, b: 0 };
}

// ============================================================
// CONSOLE
// ============================================================

pub struct Console<'a> {
    buffer: &'a mut [u8],
    info: FrameBufferInfo,

    x: usize,
    y: usize,

    fg: Color,
    bg: Color,

    cursor_visible: bool,
    cursor_saved: [Color; 16],
}

impl<'a> Console<'a> {
    // ========================================================
    // NEW
    // ========================================================

    pub fn new(buffer: &'a mut [u8], info: FrameBufferInfo) -> Self {
        Self {
            buffer,
            info,

            x: 0,
            y: 0,

            fg: Color::WHITE,
            bg: Color { r: 0, g: 0, b: 0 },

            cursor_visible: false,
            cursor_saved: [Color { r: 0, g: 0, b: 0 }; 16],
        }
    }

    // ========================================================
    // COLORS
    // ========================================================

    pub fn set_foreground(&mut self, color: Color) {
        self.fg = color;
    }

    pub fn cursor_x(&self) -> usize {
        self.x
    }

    pub fn cursor_y(&self) -> usize {
        self.y
    }

    // ========================================================
    // CLEAR
    // ========================================================

    pub fn clear(&mut self) {
        let width = self.info.width;
        let height = self.info.height;

        for y in 0..height {
            for x in 0..width {
                self.put_pixel(x, y, self.bg);
            }
        }

        self.x = 0;
        self.y = 0;

        self.cursor_visible = false;
    }

    // ========================================================
    // PRINT
    // ========================================================

    pub fn print(&mut self, text: &str, color: Color) {
        for character in text.chars() {
            self.print_char(character, color);
        }
    }

    pub fn println(&mut self, text: &str, color: Color) {
        self.print(text, color);

        self.new_line();
    }

    // ========================================================
    // CURSOR
    // ========================================================

    pub fn draw_cursor(&mut self) {
        if self.cursor_visible {
            return;
        }

        let x = self.x;
        let y = self.y;

        // Cursor'un altındaki pikselleri kaydet.
        let mut index = 0;

        for row in 6..8 {
            for col in 0..8 {
                self.cursor_saved[index] =
                    self.get_pixel(x + col, y + row);

                index += 1;
            }
        }

        // Cursor çiz.
        for row in 6..8 {
            for col in 0..8 {
                self.put_pixel(x + col, y + row, self.fg);
            }
        }

        self.cursor_visible = true;
    }

    pub fn clear_cursor(&mut self) {
        if !self.cursor_visible {
            return;
        }

        let x = self.x;
        let y = self.y;

        // Kaydedilen pikselleri geri yükle.
        let mut index = 0;

        for row in 6..8 {
            for col in 0..8 {
                let color = self.cursor_saved[index];

                self.put_pixel(x + col, y + row, color);

                index += 1;
            }
        }

        self.cursor_visible = false;
    }

    pub fn toggle_cursor(&mut self, visible: bool) {
        if visible {
            self.draw_cursor();
        } else {
            self.clear_cursor();
        }
    }

    pub fn print_char(&mut self, character: char, color: Color) {
        self.clear_cursor();
        match character {
            '\n' => {
                self.new_line();
            }

            '\r' => {
                self.x = 0;
            }

            '\t' => {
                self.x += 32;

                if self.x + 8 >= self.info.width {
                    self.new_line();
                }
            }

            '\u{0008}' => {
                self.backspace();
            }

            _ => {
                self.draw_char(character, color);
            }
        }
    }

    // ========================================================
    // BACKSPACE
    // ========================================================

    pub fn backspace(&mut self) {
        self.clear_cursor();

        if self.x == 0 {
            return;
        }

        self.x -= 8;

        for row in 0..8 {
            for col in 0..8 {
                self.put_pixel(
                    self.x + col,
                    self.y + row,
                    self.bg,
                );
            }
        }
    }

    // ========================================================
    // NEW LINE
    // ========================================================

    fn new_line(&mut self) {
        self.clear_cursor();

        self.x = 0;

        self.y += 10;

        if self.y + 8 >= self.info.height {
            self.scroll();
        }
    }

    // ========================================================
    // DRAW CHARACTER
    // ========================================================

    fn draw_char(&mut self, character: char, color: Color) {
        if self.x + 8 >= self.info.width {
            self.new_line();
        }

        let glyph_data = glyph(character);

        for row in 0..8 {
            let bits = glyph_data[row];

            for col in 0..8 {
                if bits & (1 << (7 - col)) != 0 {
                    self.put_pixel(self.x + col, self.y + row, color);
                }
            }
        }

        self.x += 8;
    }
    
    // ========================================================
    // PIXEL
    // ========================================================

    fn get_pixel(&self, x: usize, y: usize) -> Color {
        if x >= self.info.width || y >= self.info.height {
            return self.bg;
        }

        let pixel_offset =
            y * self.info.bytes_per_pixel * self.info.width
                + x * self.info.bytes_per_pixel;

        match self.info.pixel_format {
            PixelFormat::Rgb => Color {
                r: self.buffer[pixel_offset],
                g: self.buffer[pixel_offset + 1],
                b: self.buffer[pixel_offset + 2],
            },

            PixelFormat::Bgr => Color {
                r: self.buffer[pixel_offset + 2],
                g: self.buffer[pixel_offset + 1],
                b: self.buffer[pixel_offset],
            },

            _ => self.bg,
        }
    }

    fn put_pixel(&mut self, x: usize, y: usize, color: Color) {
        if x >= self.info.width || y >= self.info.height {
            return;
        }

        let pixel_offset =
            y * self.info.bytes_per_pixel * self.info.width + x * self.info.bytes_per_pixel;

        match self.info.pixel_format {
            PixelFormat::Rgb => {
                self.buffer[pixel_offset] = color.r;

                self.buffer[pixel_offset + 1] = color.g;

                self.buffer[pixel_offset + 2] = color.b;
            }

            PixelFormat::Bgr => {
                self.buffer[pixel_offset] = color.b;

                self.buffer[pixel_offset + 1] = color.g;

                self.buffer[pixel_offset + 2] = color.r;
            }

            _ => {}
        }
    }

    // ========================================================
    // SCROLL
    // ========================================================

    fn scroll(&mut self) {
        self.clear_cursor();

        let width = self.info.width;
        let height = self.info.height;

        let bytes_per_pixel = self.info.bytes_per_pixel;

        let line_height = 10;

        for y in line_height..height {
            let src = y * width * bytes_per_pixel;

            let dst = (y - line_height) * width * bytes_per_pixel;

            let length = width * bytes_per_pixel;

            self.buffer.copy_within(src..src + length, dst);
        }

        for y in height - line_height..height {
            for x in 0..width {
                self.put_pixel(x, y, self.bg);
            }
        }

        self.y = height - line_height;

        self.x = 0;
    }
}

// ============================================================
// FONT
// ============================================================

fn glyph(c: char) -> [u8; 8] {
    match c {
        // ====================================================
        // SPACE
        // ====================================================
        ' ' => [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],

        // ====================================================
        // NUMBERS
        // ====================================================
        '0' => [0x3C, 0x66, 0x6E, 0x76, 0x66, 0x66, 0x3C, 0x00],

        '1' => [0x18, 0x38, 0x18, 0x18, 0x18, 0x18, 0x7E, 0x00],

        '2' => [0x3C, 0x66, 0x06, 0x0C, 0x18, 0x30, 0x66, 0x7E],

        '3' => [0x3C, 0x66, 0x06, 0x1C, 0x06, 0x66, 0x3C, 0x00],

        '4' => [0x0C, 0x1C, 0x3C, 0x6C, 0x7E, 0x0C, 0x0C, 0x00],

        '5' => [0x7E, 0x60, 0x7C, 0x06, 0x06, 0x66, 0x3C, 0x00],

        '6' => [0x1C, 0x30, 0x60, 0x7C, 0x66, 0x66, 0x3C, 0x00],

        '7' => [0x7E, 0x66, 0x06, 0x0C, 0x18, 0x18, 0x18, 0x00],

        '8' => [0x3C, 0x66, 0x66, 0x3C, 0x66, 0x66, 0x3C, 0x00],

        '9' => [0x3C, 0x66, 0x66, 0x3E, 0x06, 0x0C, 0x38, 0x00],

        // ====================================================
        // UPPERCASE
        // ====================================================
        'A' => [0x18, 0x3C, 0x66, 0x66, 0x7E, 0x66, 0x66, 0x00],
        'B' => [0x7C, 0x66, 0x66, 0x7C, 0x66, 0x66, 0x7C, 0x00],
        'C' => [0x3C, 0x66, 0x60, 0x60, 0x60, 0x66, 0x3C, 0x00],
        'D' => [0x78, 0x6C, 0x66, 0x66, 0x66, 0x6C, 0x78, 0x00],
        'E' => [0x7E, 0x60, 0x60, 0x7C, 0x60, 0x60, 0x7E, 0x00],
        'F' => [0x7E, 0x60, 0x60, 0x7C, 0x60, 0x60, 0x60, 0x00],
        'G' => [0x3C, 0x66, 0x60, 0x6E, 0x66, 0x66, 0x3C, 0x00],
        'H' => [0x66, 0x66, 0x66, 0x7E, 0x66, 0x66, 0x66, 0x00],
        'I' => [0x7E, 0x18, 0x18, 0x18, 0x18, 0x18, 0x7E, 0x00],
        'J' => [0x1E, 0x0C, 0x0C, 0x0C, 0x0C, 0x6C, 0x38, 0x00],
        'K' => [0x66, 0x6C, 0x78, 0x70, 0x78, 0x6C, 0x66, 0x00],
        'L' => [0x60, 0x60, 0x60, 0x60, 0x60, 0x60, 0x7E, 0x00],
        'M' => [0x63, 0x77, 0x7F, 0x6B, 0x63, 0x63, 0x63, 0x00],
        'N' => [0x66, 0x76, 0x7E, 0x7E, 0x6E, 0x66, 0x66, 0x00],
        'O' => [0x3C, 0x66, 0x66, 0x66, 0x66, 0x66, 0x3C, 0x00],
        'P' => [0x7C, 0x66, 0x66, 0x7C, 0x60, 0x60, 0x60, 0x00],
        'Q' => [0x3C, 0x66, 0x66, 0x66, 0x6E, 0x3C, 0x0E, 0x00],
        'R' => [0x7C, 0x66, 0x66, 0x7C, 0x78, 0x6C, 0x66, 0x00],
        'S' => [0x3C, 0x66, 0x60, 0x3C, 0x06, 0x66, 0x3C, 0x00],
        'T' => [0x7E, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x00],
        'U' => [0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x3C, 0x00],
        'V' => [0x66, 0x66, 0x66, 0x66, 0x66, 0x3C, 0x18, 0x00],
        'W' => [0x63, 0x63, 0x63, 0x6B, 0x7F, 0x77, 0x63, 0x00],
        'X' => [0x66, 0x66, 0x3C, 0x18, 0x3C, 0x66, 0x66, 0x00],
        'Y' => [0x66, 0x66, 0x3C, 0x18, 0x18, 0x18, 0x18, 0x00],
        'Z' => [0x7E, 0x06, 0x0C, 0x18, 0x30, 0x60, 0x7E, 0x00],

        // ====================================================
        // LOWERCASE
        // ====================================================
        'a' => [0x00, 0x00, 0x3C, 0x06, 0x3E, 0x66, 0x3E, 0x00],
        'b' => [0x60, 0x60, 0x7C, 0x66, 0x66, 0x66, 0x7C, 0x00],
        'c' => [0x00, 0x00, 0x3C, 0x66, 0x60, 0x66, 0x3C, 0x00],
        'd' => [0x06, 0x06, 0x3E, 0x66, 0x66, 0x66, 0x3E, 0x00],
        'e' => [0x00, 0x00, 0x3C, 0x66, 0x7E, 0x60, 0x3C, 0x00],
        'f' => [0x1C, 0x36, 0x30, 0x7C, 0x30, 0x30, 0x30, 0x00],
        'g' => [0x00, 0x00, 0x3E, 0x66, 0x66, 0x3E, 0x06, 0x7C],
        'h' => [0x60, 0x60, 0x7C, 0x66, 0x66, 0x66, 0x66, 0x00],
        'i' => [0x18, 0x00, 0x3C, 0x18, 0x18, 0x18, 0x3C, 0x00],
        'j' => [0x0C, 0x00, 0x1C, 0x0C, 0x0C, 0x0C, 0x6C, 0x38],
        'k' => [0x60, 0x60, 0x66, 0x6C, 0x78, 0x6C, 0x66, 0x00],
        'l' => [0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x3C],
        'm' => [0x00, 0x00, 0x66, 0x7F, 0x7F, 0x6B, 0x6B, 0x00],
        'n' => [0x00, 0x00, 0x7C, 0x66, 0x66, 0x66, 0x66, 0x00],
        'o' => [0x00, 0x00, 0x3C, 0x66, 0x66, 0x66, 0x3C, 0x00],
        'p' => [0x00, 0x00, 0x7C, 0x66, 0x66, 0x7C, 0x60, 0x60],
        'q' => [0x00, 0x00, 0x3E, 0x66, 0x66, 0x3E, 0x06, 0x06],
        'r' => [0x00, 0x00, 0x6C, 0x76, 0x60, 0x60, 0x60, 0x00],
        's' => [0x00, 0x00, 0x3E, 0x60, 0x3C, 0x06, 0x7C, 0x00],
        't' => [0x30, 0x30, 0x7C, 0x30, 0x30, 0x36, 0x1C, 0x00],
        'u' => [0x00, 0x00, 0x66, 0x66, 0x66, 0x66, 0x3E, 0x00],
        'v' => [0x00, 0x00, 0x66, 0x66, 0x66, 0x66, 0x3C, 0x18],
        'w' => [0x00, 0x00, 0x63, 0x6B, 0x7F, 0x7F, 0x36, 0x00],
        'x' => [0x00, 0x00, 0x66, 0x3C, 0x18, 0x3C, 0x66, 0x00],
        'y' => [0x00, 0x00, 0x66, 0x66, 0x66, 0x3E, 0x06, 0x3C],
        'z' => [0x00, 0x00, 0x7E, 0x0C, 0x18, 0x30, 0x7E, 0x00],

        // ====================================================
        // TURKISH LOWERCASE
        // ====================================================
        'ı' => [0x00, 0x00, 0x18, 0x18, 0x18, 0x18, 0x18, 0x3C],

        'ğ' => [0x18, 0x24, 0x00, 0x3C, 0x66, 0x66, 0x3E, 0x00],

        'ü' => [0x24, 0x00, 0x66, 0x66, 0x66, 0x66, 0x3E, 0x00],

        'ş' => [0x18, 0x00, 0x3E, 0x60, 0x3C, 0x06, 0x7C, 0x00],

        'ö' => [0x24, 0x00, 0x3C, 0x66, 0x66, 0x66, 0x3C, 0x00],

        'ç' => [0x00, 0x00, 0x3C, 0x66, 0x60, 0x66, 0x3C, 0x18],
        // ====================================================
        // TURKISH UPPERCASE
        // ====================================================
        'İ' => [0x18, 0x00, 0x7E, 0x18, 0x18, 0x18, 0x7E, 0x00],

        'Ğ' => [0x18, 0x24, 0x3C, 0x66, 0x60, 0x6E, 0x66, 0x00],

        'Ü' => [0x24, 0x00, 0x66, 0x66, 0x66, 0x66, 0x66, 0x3C],

        'Ş' => [0x18, 0x3C, 0x66, 0x60, 0x3C, 0x06, 0x66, 0x00],

        'Ö' => [0x24, 0x00, 0x3C, 0x66, 0x66, 0x66, 0x66, 0x3C],

        'Ç' => [0x3C, 0x66, 0x60, 0x60, 0x60, 0x66, 0x3C, 0x18],

        // ====================================================
        // SYMBOLS
        // ====================================================

        '!' => [0x18, 0x18, 0x18, 0x18, 0x18, 0x00, 0x18, 0x00],

        '?' => [0x3C, 0x66, 0x06, 0x0C, 0x18, 0x18, 0x00, 0x18],

        '.' => [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x00],

        ',' => [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x30],

        ':' => [0x00, 0x18, 0x18, 0x00, 0x00, 0x18, 0x18, 0x00],

        ';' => [0x00, 0x18, 0x18, 0x00, 0x00, 0x18, 0x18, 0x30],

        '-' => [0x00, 0x00, 0x00, 0x7E, 0x00, 0x00, 0x00, 0x00],

        '_' => [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF],

        '+' => [0x00, 0x18, 0x18, 0x7E, 0x18, 0x18, 0x00, 0x00],

        '=' => [0x00, 0x00, 0x7E, 0x00, 0x7E, 0x00, 0x00, 0x00],

        '*' => [0x00, 0x66, 0x3C, 0xFF, 0x3C, 0x66, 0x00, 0x00],

        '/' => [0x06, 0x0C, 0x18, 0x30, 0x60, 0xC0, 0x80, 0x00],

        '\\' => [0xC0, 0x60, 0x30, 0x18, 0x0C, 0x06, 0x02, 0x00],

        '(' => [0x0C, 0x18, 0x30, 0x30, 0x30, 0x30, 0x18, 0x0C],

        ')' => [0x30, 0x18, 0x0C, 0x0C, 0x0C, 0x0C, 0x18, 0x30],

        '[' => [0x3C, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x3C],

        ']' => [0x3C, 0x0C, 0x0C, 0x0C, 0x0C, 0x0C, 0x0C, 0x3C],

        '<' => [0x0C, 0x18, 0x30, 0x60, 0x30, 0x18, 0x0C, 0x00],

        '>' => [0x30, 0x18, 0x0C, 0x06, 0x0C, 0x18, 0x30, 0x00],

        '|' => [0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18],

        '"' => [0x66, 0x66, 0x66, 0x24, 0x00, 0x00, 0x00, 0x00],

        '\'' => [0x18, 0x18, 0x18, 0x10, 0x00, 0x00, 0x00, 0x00],

        '@' => [0x3C, 0x66, 0x6F, 0x69, 0x6F, 0x60, 0x66, 0x3C],

        '#' => [0x24, 0x24, 0x7E, 0x24, 0x7E, 0x24, 0x24, 0x00],

        '$' => [0x18, 0x3E, 0x60, 0x3C, 0x06, 0x7C, 0x18, 0x00],

        '%' => [0x62, 0x66, 0x0C, 0x18, 0x30, 0x66, 0x46, 0x00],

        '&' => [0x38, 0x6C, 0x38, 0x76, 0xCC, 0xCC, 0x76, 0x00],

        // ====================================================
        // FALLBACK
        // ====================================================
        _ => [0x7E, 0x42, 0x5A, 0x5A, 0x5A, 0x42, 0x7E, 0x00],
    }
}
