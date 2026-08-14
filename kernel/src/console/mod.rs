use bootloader_api::info::{FrameBufferInfo, PixelFormat};

// ============================================================================
// CONSOLE GEOMETRY
// ============================================================================
//
// Fixed-width terminal:
//
//   Cell   : 8 x 16 pixels
//   Glyph  : 8 x 16 pixels
//   Tab    : 4 cells
//
// Every character occupies exactly one 8x16 cell.
// This prevents inconsistent horizontal spacing and keeps the terminal
// visually aligned across the entire framebuffer.
//
// ============================================================================

const CELL_WIDTH: usize = 8;
const CELL_HEIGHT: usize = 16;

const GLYPH_WIDTH: usize = 8;
const GLYPH_HEIGHT: usize = 16;

const TAB_SIZE: usize = 4;
const CURSOR_HEIGHT: usize = 2;

// ============================================================================
// COLOR
// ============================================================================

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const BLACK: Self = Self::rgb(0, 0, 0);
    pub const WHITE: Self = Self::rgb(255, 255, 255);

    pub const GREEN: Self = Self::rgb(0, 255, 0);
    pub const BLUE: Self = Self::rgb(0, 100, 255);
    pub const RED: Self = Self::rgb(255, 0, 0);

    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

// ============================================================================
// CONSOLE
// ============================================================================

pub struct Console<'a> {
    buffer: &'a mut [u8],
    info: FrameBufferInfo,

    cursor_x: usize,
    cursor_y: usize,

    foreground: Color,
    background: Color,

    cursor_visible: bool,
    cursor_saved: [Color; GLYPH_WIDTH * CURSOR_HEIGHT],
}

impl<'a> Console<'a> {
    // ========================================================================
    // CONSTRUCTOR
    // ========================================================================

    pub fn new(
        buffer: &'a mut [u8],
        info: FrameBufferInfo,
    ) -> Self {
        Self {
            buffer,
            info,

            cursor_x: 0,
            cursor_y: 0,

            foreground: Color::WHITE,
            background: Color::BLACK,

            cursor_visible: false,
            cursor_saved: [Color::BLACK; GLYPH_WIDTH * CURSOR_HEIGHT],
        }
    }

    // ========================================================================
    // COLOR
    // ========================================================================

    pub fn set_foreground(&mut self, color: Color) {
        self.foreground = color;
    }

    pub fn set_background(&mut self, color: Color) {
        self.background = color;
    }

    pub fn foreground(&self) -> Color {
        self.foreground
    }

    pub fn background(&self) -> Color {
        self.background
    }

    // ========================================================================
    // POSITION
    // ========================================================================

    pub fn cursor_x(&self) -> usize {
        self.cursor_x
    }

    pub fn cursor_y(&self) -> usize {
        self.cursor_y
    }

    pub fn width(&self) -> usize {
        self.info.width
    }

    pub fn height(&self) -> usize {
        self.info.height
    }

    pub fn columns(&self) -> usize {
        self.info.width / CELL_WIDTH
    }

    pub fn rows(&self) -> usize {
        self.info.height / CELL_HEIGHT
    }

    // ========================================================================
    // CLEAR
    // ========================================================================

    pub fn clear(&mut self) {
        self.clear_cursor();

        let width = self.info.width;
        let height = self.info.height;
        let background = self.background;

        for y in 0..height {
            for x in 0..width {
                self.put_pixel(x, y, background);
            }
        }

        self.cursor_x = 0;
        self.cursor_y = 0;
    }

    // ========================================================================
    // PRINT
    // ========================================================================

    pub fn print(&mut self, text: &str, color: Color) {
        for character in text.chars() {
            self.print_char(character, color);
        }
    }

    pub fn println(&mut self, text: &str, color: Color) {
        self.print(text, color);
        self.new_line();
    }

    // ========================================================================
    // CHARACTER OUTPUT
    // ========================================================================

    pub fn print_char(
        &mut self,
        character: char,
        color: Color,
    ) {
        self.clear_cursor();

        match character {
            '\n' => self.new_line(),

            '\r' => {
                self.cursor_x = 0;
            }

            '\t' => self.advance_tab(),

            '\u{0008}' => self.backspace(),

            _ => self.draw_char(character, color),
        }
    }

    // ========================================================================
    // TAB
    // ========================================================================

    fn advance_tab(&mut self) {
        let current_column = self.cursor_x / CELL_WIDTH;
        let next_column =
            ((current_column / TAB_SIZE) + 1) * TAB_SIZE;

        if next_column >= self.columns() {
            self.new_line();
        } else {
            self.cursor_x = next_column * CELL_WIDTH;
        }
    }

    // ========================================================================
    // BACKSPACE
    // ========================================================================

    pub fn backspace(&mut self) {
        self.clear_cursor();

        if self.cursor_x == 0 {
            return;
        }

        self.cursor_x =
            self.cursor_x.saturating_sub(CELL_WIDTH);

        self.clear_cell();
    }

    // ========================================================================
    // NEW LINE
    // ========================================================================

    fn new_line(&mut self) {
        self.clear_cursor();

        self.cursor_x = 0;
        self.cursor_y += CELL_HEIGHT;

        if self.cursor_y + GLYPH_HEIGHT > self.info.height {
            self.scroll();
        }
    }

    // ========================================================================
    // DRAW CHARACTER
    // ========================================================================

    fn draw_char(
        &mut self,
        character: char,
        color: Color,
    ) {
        // Horizontal wrapping.
        if self.cursor_x + CELL_WIDTH > self.info.width {
            self.new_line();
        }

        // Vertical protection.
        if self.cursor_y + GLYPH_HEIGHT > self.info.height {
            self.scroll();
        }

        // Clear the complete character cell first.
        self.clear_cell();

        let bitmap = glyph(character);

        for row in 0..GLYPH_HEIGHT {
            let bits = bitmap[row];

            for column in 0..GLYPH_WIDTH {
                if bits & (0x80 >> column) != 0 {
                    self.put_pixel(
                        self.cursor_x + column,
                        self.cursor_y + row,
                        color,
                    );
                }
            }
        }

        // Every character advances exactly one cell.
        self.cursor_x += CELL_WIDTH;
    }

    // ========================================================================
    // CLEAR CELL
    // ========================================================================

    fn clear_cell(&mut self) {
        if self.cursor_x >= self.info.width
            || self.cursor_y >= self.info.height
        {
            return;
        }

        let end_x =
            (self.cursor_x + CELL_WIDTH).min(self.info.width);

        let end_y =
            (self.cursor_y + CELL_HEIGHT).min(self.info.height);

        let background = self.background;

        for y in self.cursor_y..end_y {
            for x in self.cursor_x..end_x {
                self.put_pixel(x, y, background);
            }
        }
    }

    // ========================================================================
    // CURSOR
    // ========================================================================

    pub fn draw_cursor(&mut self) {
        if self.cursor_visible {
            return;
        }

        if self.cursor_x + GLYPH_WIDTH > self.info.width
            || self.cursor_y + GLYPH_HEIGHT > self.info.height
        {
            return;
        }

        let cursor_y =
            self.cursor_y + GLYPH_HEIGHT - CURSOR_HEIGHT;

        let mut index = 0;

        // Save the pixels underneath the cursor.
        for row in 0..CURSOR_HEIGHT {
            for column in 0..GLYPH_WIDTH {
                self.cursor_saved[index] =
                    self.get_pixel(
                        self.cursor_x + column,
                        cursor_y + row,
                    );

                index += 1;
            }
        }

        // Draw cursor.
        for row in 0..CURSOR_HEIGHT {
            for column in 0..GLYPH_WIDTH {
                self.put_pixel(
                    self.cursor_x + column,
                    cursor_y + row,
                    self.foreground,
                );
            }
        }

        self.cursor_visible = true;
    }

    pub fn clear_cursor(&mut self) {
        if !self.cursor_visible {
            return;
        }

        let cursor_y =
            self.cursor_y + GLYPH_HEIGHT - CURSOR_HEIGHT;

        let mut index = 0;

        for row in 0..CURSOR_HEIGHT {
            for column in 0..GLYPH_WIDTH {
                let color = self.cursor_saved[index];

                self.put_pixel(
                    self.cursor_x + column,
                    cursor_y + row,
                    color,
                );

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

    // ========================================================================
    // PIXEL READ
    // ========================================================================

    fn get_pixel(
        &self,
        x: usize,
        y: usize,
    ) -> Color {
        if x >= self.info.width
            || y >= self.info.height
        {
            return self.background;
        }

        let offset =
            y * self.info.stride * self.info.bytes_per_pixel
                + x * self.info.bytes_per_pixel;

        match self.info.pixel_format {
            PixelFormat::Rgb => Color {
                r: self.buffer[offset],
                g: self.buffer[offset + 1],
                b: self.buffer[offset + 2],
            },

            PixelFormat::Bgr => Color {
                r: self.buffer[offset + 2],
                g: self.buffer[offset + 1],
                b: self.buffer[offset],
            },

            _ => self.background,
        }
    }

    // ========================================================================
    // PIXEL WRITE
    // ========================================================================

    fn put_pixel(
        &mut self,
        x: usize,
        y: usize,
        color: Color,
    ) {
        if x >= self.info.width
            || y >= self.info.height
        {
            return;
        }

        let offset =
            y * self.info.stride * self.info.bytes_per_pixel
                + x * self.info.bytes_per_pixel;

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

            _ => {}
        }
    }

    // ========================================================================
    // SCROLL
    // ========================================================================

    fn scroll(&mut self) {
        self.clear_cursor();

        let width = self.info.width;
        let height = self.info.height;
        let stride = self.info.stride;
        let bytes_per_pixel = self.info.bytes_per_pixel;

        let line_bytes =
            CELL_HEIGHT * stride * bytes_per_pixel;

        let total_bytes =
            height * stride * bytes_per_pixel;

        if line_bytes >= total_bytes {
            self.clear();
            return;
        }

        // Move framebuffer contents upward by one terminal row.
        self.buffer.copy_within(
            line_bytes..total_bytes,
            0,
        );

        // Clear the newly exposed bottom row.
        let background = self.background;
        let start_y = height - CELL_HEIGHT;

        for y in start_y..height {
            for x in 0..width {
                self.put_pixel(x, y, background);
            }
        }

        self.cursor_x = 0;
        self.cursor_y = start_y;
    }
}

// ============================================================================
// 8x16 BITMAP FONT
// ============================================================================
//
// Every glyph:
//
//     [u8; 16]
//
// Every row contains exactly 8 pixels.
//
// Bit 7 = leftmost pixel
// Bit 0 = rightmost pixel
//
// IMPORTANT:
// The terminal geometry is fixed at 8x16.
// Individual glyphs may intentionally use different vertical ink heights
// (for example lowercase letters), but every glyph occupies the same cell.
//
// ============================================================================

fn glyph(c: char) -> [u8; 16] {
    match c {
        // ====================================================================
        // SPACE
        // ====================================================================

        ' ' => [
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ],

        // ====================================================================
        // NUMBERS
        // ====================================================================

        '0' => [
            0x3C, 0x66, 0x66, 0x6E,
            0x76, 0x66, 0x66, 0x66,
            0x66, 0x66, 0x66, 0x66,
            0x3C, 0x00, 0x00, 0x00,
        ],

        '1' => [
            0x18, 0x38, 0x78, 0x18,
            0x18, 0x18, 0x18, 0x18,
            0x18, 0x18, 0x18, 0x18,
            0x7E, 0x00, 0x00, 0x00,
        ],

        '2' => [
            0x3C, 0x66, 0x06, 0x06,
            0x0C, 0x18, 0x30, 0x60,
            0x60, 0x60, 0x60, 0x66,
            0x7E, 0x00, 0x00, 0x00,
        ],

        '3' => [
            0x3C, 0x66, 0x06, 0x06,
            0x0C, 0x1C, 0x06, 0x06,
            0x06, 0x06, 0x06, 0x66,
            0x3C, 0x00, 0x00, 0x00,
        ],

        '4' => [
            0x0C, 0x1C, 0x3C, 0x6C,
            0x6C, 0xCC, 0xCC, 0xFE,
            0x0C, 0x0C, 0x0C, 0x0C,
            0x0C, 0x00, 0x00, 0x00,
        ],

        '5' => [
            0x7E, 0x60, 0x60, 0x60,
            0x7C, 0x66, 0x06, 0x06,
            0x06, 0x06, 0x06, 0x66,
            0x3C, 0x00, 0x00, 0x00,
        ],

        '6' => [
            0x1C, 0x30, 0x60, 0x60,
            0x7C, 0x66, 0x66, 0x66,
            0x66, 0x66, 0x66, 0x66,
            0x3C, 0x00, 0x00, 0x00,
        ],

        '7' => [
            0x7E, 0x06, 0x06, 0x0C,
            0x18, 0x18, 0x30, 0x30,
            0x30, 0x60, 0x60, 0x60,
            0x60, 0x00, 0x00, 0x00,
        ],

        '8' => [
            0x3C, 0x66, 0x66, 0x66,
            0x66, 0x3C, 0x66, 0x66,
            0x66, 0x66, 0x66, 0x66,
            0x3C, 0x00, 0x00, 0x00,
        ],

        '9' => [
            0x3C, 0x66, 0x66, 0x66,
            0x66, 0x66, 0x66, 0x3E,
            0x06, 0x06, 0x06, 0x0C,
            0x38, 0x00, 0x00, 0x00,
        ],

        // ====================================================================
        // UPPERCASE
        // ====================================================================

        'A' => [
            0x18, 0x3C, 0x66, 0x66,
            0x66, 0x66, 0x7E, 0x66,
            0x66, 0x66, 0x66, 0x66,
            0x66, 0x00, 0x00, 0x00,
        ],

        'B' => [
            0x7C, 0x66, 0x66, 0x66,
            0x66, 0x7C, 0x66, 0x66,
            0x66, 0x66, 0x66, 0x66,
            0x7C, 0x00, 0x00, 0x00,
        ],

        'C' => [
            0x3C, 0x66, 0x66, 0x60,
            0x60, 0x60, 0x60, 0x60,
            0x60, 0x60, 0x60, 0x66,
            0x3C, 0x00, 0x00, 0x00,
        ],

        'D' => [
            0x78, 0x6C, 0x66, 0x66,
            0x66, 0x66, 0x66, 0x66,
            0x66, 0x66, 0x66, 0x6C,
            0x78, 0x00, 0x00, 0x00,
        ],

        'E' => [
            0x7E, 0x60, 0x60, 0x60,
            0x60, 0x7C, 0x60, 0x60,
            0x60, 0x60, 0x60, 0x60,
            0x7E, 0x00, 0x00, 0x00,
        ],

        'F' => [
            0x7E, 0x60, 0x60, 0x60,
            0x60, 0x7C, 0x60, 0x60,
            0x60, 0x60, 0x60, 0x60,
            0x60, 0x00, 0x00, 0x00,
        ],

        'G' => [
            0x3C, 0x66, 0x66, 0x60,
            0x60, 0x6E, 0x66, 0x66,
            0x66, 0x66, 0x66, 0x66,
            0x3E, 0x00, 0x00, 0x00,
        ],

        'H' => [
            0x66, 0x66, 0x66, 0x66,
            0x66, 0x7E, 0x66, 0x66,
            0x66, 0x66, 0x66, 0x66,
            0x66, 0x00, 0x00, 0x00,
        ],

        'I' => [
            0x3C, 0x18, 0x18, 0x18,
            0x18, 0x18, 0x18, 0x18,
            0x18, 0x18, 0x18, 0x18,
            0x3C, 0x00, 0x00, 0x00,
        ],

        'J' => [
            0x1E, 0x06, 0x06, 0x06,
            0x06, 0x06, 0x06, 0x06,
            0x06, 0x06, 0x66, 0x66,
            0x3C, 0x00, 0x00, 0x00,
        ],

        'K' => [
            0x66, 0x66, 0x6C, 0x6C,
            0x78, 0x78, 0x70, 0x78,
            0x78, 0x6C, 0x6C, 0x66,
            0x66, 0x00, 0x00, 0x00,
        ],

        'L' => [
            0x60, 0x60, 0x60, 0x60,
            0x60, 0x60, 0x60, 0x60,
            0x60, 0x60, 0x60, 0x60,
            0x7E, 0x00, 0x00, 0x00,
        ],

        'M' => [
            0xC6, 0xEE, 0xFE, 0xD6,
            0xC6, 0xC6, 0xC6, 0xC6,
            0xC6, 0xC6, 0xC6, 0xC6,
            0xC6, 0x00, 0x00, 0x00,
        ],

        'N' => [
            0x66, 0x76, 0x7E, 0x7E,
            0x6E, 0x6E, 0x66, 0x66,
            0x66, 0x66, 0x66, 0x66,
            0x66, 0x00, 0x00, 0x00,
        ],

        'O' => [
            0x3C, 0x66, 0x66, 0x66,
            0x66, 0x66, 0x66, 0x66,
            0x66, 0x66, 0x66, 0x66,
            0x3C, 0x00, 0x00, 0x00,
        ],

        'P' => [
            0x7C, 0x66, 0x66, 0x66,
            0x66, 0x7C, 0x60, 0x60,
            0x60, 0x60, 0x60, 0x60,
            0x60, 0x00, 0x00, 0x00,
        ],

        'Q' => [
            0x3C, 0x66, 0x66, 0x66,
            0x66, 0x66, 0x66, 0x66,
            0x66, 0x6E, 0x6C, 0x3A,
            0x06, 0x00, 0x00, 0x00,
        ],

        'R' => [
            0x7C, 0x66, 0x66, 0x66,
            0x66, 0x7C, 0x6C, 0x66,
            0x66, 0x66, 0x66, 0x66,
            0x66, 0x00, 0x00, 0x00,
        ],

        'S' => [
            0x3C, 0x66, 0x60, 0x60,
            0x60, 0x30, 0x18, 0x0C,
            0x06, 0x06, 0x06, 0x66,
            0x3C, 0x00, 0x00, 0x00,
        ],

        'T' => [
            0x7E, 0x18, 0x18, 0x18,
            0x18, 0x18, 0x18, 0x18,
            0x18, 0x18, 0x18, 0x18,
            0x18, 0x00, 0x00, 0x00,
        ],

        'U' => [
            0x66, 0x66, 0x66, 0x66,
            0x66, 0x66, 0x66, 0x66,
            0x66, 0x66, 0x66, 0x66,
            0x3C, 0x00, 0x00, 0x00,
        ],

        'V' => [
            0x66, 0x66, 0x66, 0x66,
            0x66, 0x66, 0x66, 0x66,
            0x66, 0x66, 0x3C, 0x3C,
            0x18, 0x00, 0x00, 0x00,
        ],

        'W' => [
            0xC6, 0xC6, 0xC6, 0xC6,
            0xC6, 0xC6, 0xD6, 0xD6,
            0xFE, 0xEE, 0xC6, 0xC6,
            0xC6, 0x00, 0x00, 0x00,
        ],

        'X' => [
            0x66, 0x66, 0x3C, 0x3C,
            0x18, 0x18, 0x18, 0x18,
            0x3C, 0x3C, 0x66, 0x66,
            0x66, 0x00, 0x00, 0x00,
        ],

        'Y' => [
            0x66, 0x66, 0x66, 0x66,
            0x3C, 0x3C, 0x18, 0x18,
            0x18, 0x18, 0x18, 0x18,
            0x18, 0x00, 0x00, 0x00,
        ],

        'Z' => [
            0x7E, 0x06, 0x0C, 0x18,
            0x18, 0x30, 0x30, 0x60,
            0x60, 0x60, 0x60, 0x7E,
            0x00, 0x00, 0x00, 0x00,
        ],

        // ====================================================================
        // LOWERCASE
        // ====================================================================

        'a' => [
            0x00, 0x00, 0x00, 0x00,
            0x3C, 0x06, 0x3E, 0x66,
            0x66, 0x66, 0x66, 0x3E,
            0x00, 0x00, 0x00, 0x00,
        ],

        'b' => [
            0x60, 0x60, 0x60, 0x60,
            0x7C, 0x66, 0x66, 0x66,
            0x66, 0x66, 0x66, 0x7C,
            0x00, 0x00, 0x00, 0x00,
        ],

        'c' => [
            0x00, 0x00, 0x00, 0x00,
            0x3C, 0x66, 0x60, 0x60,
            0x60, 0x60, 0x66, 0x3C,
            0x00, 0x00, 0x00, 0x00,
        ],

        'd' => [
            0x06, 0x06, 0x06, 0x06,
            0x3E, 0x66, 0x66, 0x66,
            0x66, 0x66, 0x66, 0x3E,
            0x00, 0x00, 0x00, 0x00,
        ],

        'e' => [
            0x00, 0x00, 0x00, 0x00,
            0x3C, 0x66, 0x7E, 0x60,
            0x60, 0x60, 0x66, 0x3C,
            0x00, 0x00, 0x00, 0x00,
        ],

        'f' => [
            0x1C, 0x36, 0x30, 0x30,
            0x7E, 0x30, 0x30, 0x30,
            0x30, 0x30, 0x30, 0x30,
            0x00, 0x00, 0x00, 0x00,
        ],

        'g' => [
            0x00, 0x00, 0x00, 0x00,
            0x3E, 0x66, 0x66, 0x66,
            0x66, 0x66, 0x3E, 0x06,
            0x66, 0x3C, 0x00, 0x00,
        ],

        'h' => [
            0x60, 0x60, 0x60, 0x60,
            0x7C, 0x66, 0x66, 0x66,
            0x66, 0x66, 0x66, 0x66,
            0x00, 0x00, 0x00, 0x00,
        ],

        'i' => [
            0x18, 0x00, 0x38, 0x18,
            0x18, 0x18, 0x18, 0x18,
            0x18, 0x18, 0x3C, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ],

        'j' => [
            0x06, 0x00, 0x0E, 0x06,
            0x06, 0x06, 0x06, 0x06,
            0x66, 0x66, 0x3C, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ],

        'k' => [
            0x60, 0x60, 0x60, 0x60,
            0x66, 0x6C, 0x78, 0x70,
            0x78, 0x6C, 0x66, 0x66,
            0x00, 0x00, 0x00, 0x00,
        ],

        'l' => [
            0x38, 0x18, 0x18, 0x18,
            0x18, 0x18, 0x18, 0x18,
            0x18, 0x18, 0x3C, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ],

        'm' => [
            0x00, 0x00, 0x00, 0x00,
            0x6C, 0x7E, 0x7E, 0x66,
            0x66, 0x66, 0x66, 0x66,
            0x00, 0x00, 0x00, 0x00,
        ],

        'n' => [
            0x00, 0x00, 0x00, 0x00,
            0x7C, 0x66, 0x66, 0x66,
            0x66, 0x66, 0x66, 0x66,
            0x00, 0x00, 0x00, 0x00,
        ],

        'o' => [
            0x00, 0x00, 0x00, 0x00,
            0x3C, 0x66, 0x66, 0x66,
            0x66, 0x66, 0x66, 0x3C,
            0x00, 0x00, 0x00, 0x00,
        ],

        'p' => [
            0x00, 0x00, 0x00, 0x00,
            0x7C, 0x66, 0x66, 0x66,
            0x66, 0x7C, 0x60, 0x60,
            0x60, 0x00, 0x00, 0x00,
        ],

        'q' => [
            0x00, 0x00, 0x00, 0x00,
            0x3E, 0x66, 0x66, 0x66,
            0x66, 0x3E, 0x06, 0x06,
            0x06, 0x00, 0x00, 0x00,
        ],

        'r' => [
            0x00, 0x00, 0x00, 0x00,
            0x6C, 0x76, 0x60, 0x60,
            0x60, 0x60, 0x60, 0x60,
            0x00, 0x00, 0x00, 0x00,
        ],

        's' => [
            0x00, 0x00, 0x00, 0x00,
            0x3E, 0x60, 0x60, 0x3C,
            0x06, 0x06, 0x06, 0x7C,
            0x00, 0x00, 0x00, 0x00,
        ],

        't' => [
            0x00, 0x30, 0x30, 0x7E,
            0x30, 0x30, 0x30, 0x30,
            0x30, 0x36, 0x1C, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ],
        'u' => [
            0x00, 0x00, 0x00, 0x00,
            0x66, 0x66, 0x66, 0x66,
            0x66, 0x66, 0x66, 0x3E,
            0x00, 0x00, 0x00, 0x00,
        ],

        'v' => [
            0x00, 0x00, 0x00, 0x00,
            0x66, 0x66, 0x66, 0x66,
            0x66, 0x66, 0x3C, 0x18,
            0x00, 0x00, 0x00, 0x00,
        ],

        'w' => [
            0x00, 0x00, 0x00, 0x00,
            0x66, 0x66, 0x6E, 0x6E,
            0x7E, 0x76, 0x66, 0x66,
            0x00, 0x00, 0x00, 0x00,
        ],

        'x' => [
            0x00, 0x00, 0x00, 0x00,
            0x66, 0x66, 0x3C, 0x18,
            0x18, 0x3C, 0x66, 0x66,
            0x00, 0x00, 0x00, 0x00,
        ],

        'y' => [
            0x00, 0x00, 0x00, 0x00,
            0x66, 0x66, 0x66, 0x66,
            0x3E, 0x06, 0x0C, 0x78,
            0x00, 0x00, 0x00, 0x00,
        ],

        'z' => [
            0x00, 0x00, 0x00, 0x00,
            0x7E, 0x0C, 0x18, 0x30,
            0x60, 0x60, 0x7E, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ],

        // ====================================================================
        // TURKISH LOWERCASE
        // ====================================================================

        'ı' => [
            0x00, 0x00, 0x00, 0x00,
            0x18, 0x18, 0x18, 0x18,
            0x18, 0x18, 0x18, 0x3C,
            0x00, 0x00, 0x00, 0x00,
        ],

        'ğ' => [
            0x18, 0x24, 0x00, 0x00,
            0x3E, 0x66, 0x66, 0x66,
            0x66, 0x66, 0x3E, 0x06,
            0x66, 0x3C, 0x00, 0x00,
        ],

        'ü' => [
            0x24, 0x00, 0x00, 0x00,
            0x66, 0x66, 0x66, 0x66,
            0x66, 0x66, 0x66, 0x3E,
            0x00, 0x00, 0x00, 0x00,
        ],

        'ş' => [
            0x00, 0x18, 0x00, 0x00,
            0x3E, 0x60, 0x60, 0x3C,
            0x06, 0x06, 0x06, 0x7C,
            0x18, 0x00, 0x00, 0x00,
        ],

        'ö' => [
            0x24, 0x00, 0x00, 0x00,
            0x3C, 0x66, 0x66, 0x66,
            0x66, 0x66, 0x66, 0x3C,
            0x00, 0x00, 0x00, 0x00,
        ],

        'ç' => [
            0x00, 0x00, 0x00, 0x00,
            0x3C, 0x66, 0x60, 0x60,
            0x60, 0x60, 0x66, 0x3C,
            0x18, 0x00, 0x00, 0x00,
        ],

        // ====================================================================
        // TURKISH UPPERCASE
        // ====================================================================

        'İ' => [
            0x18, 0x00, 0x00, 0x00,
            0x3C, 0x18, 0x18, 0x18,
            0x18, 0x18, 0x18, 0x18,
            0x3C, 0x00, 0x00, 0x00,
        ],

        'Ğ' => [
            0x18, 0x24, 0x3C, 0x66,
            0x66, 0x60, 0x60, 0x6E,
            0x66, 0x66, 0x66, 0x3E,
            0x00, 0x00, 0x00, 0x00,
        ],

        'Ü' => [
            0x24, 0x00, 0x66, 0x66,
            0x66, 0x66, 0x66, 0x66,
            0x66, 0x66, 0x66, 0x66,
            0x3C, 0x00, 0x00, 0x00,
        ],

        'Ş' => [
            0x18, 0x3C, 0x66, 0x60,
            0x60, 0x30, 0x18, 0x0C,
            0x06, 0x06, 0x06, 0x66,
            0x3C, 0x18, 0x00, 0x00,
        ],

        'Ö' => [
            0x24, 0x00, 0x3C, 0x66,
            0x66, 0x66, 0x66, 0x66,
            0x66, 0x66, 0x66, 0x66,
            0x3C, 0x00, 0x00, 0x00,
        ],

        'Ç' => [
            0x3C, 0x66, 0x66, 0x60,
            0x60, 0x60, 0x60, 0x60,
            0x60, 0x60, 0x66, 0x3C,
            0x18, 0x00, 0x00, 0x00,
        ],

        // ====================================================================
        // PUNCTUATION
        // ====================================================================

        '!' => [
            0x18, 0x18, 0x18, 0x18,
            0x18, 0x18, 0x18, 0x18,
            0x18, 0x00, 0x18, 0x18,
            0x00, 0x00, 0x00, 0x00,
        ],

        '?' => [
            0x3C, 0x66, 0x06, 0x0C,
            0x18, 0x30, 0x30, 0x30,
            0x00, 0x30, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ],

        '.' => [
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x18, 0x18,
            0x00, 0x00, 0x00, 0x00,
        ],

        ',' => [
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x18, 0x18,
            0x30, 0x00, 0x00, 0x00,
        ],

        ':' => [
            0x00, 0x18, 0x18, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x18, 0x18, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ],

        ';' => [
            0x00, 0x18, 0x18, 0x00,
            0x00, 0x00, 0x00, 0x18,
            0x18, 0x30, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ],

        '-' => [
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x7E, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ],

        '_' => [
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x7E, 0x00, 0x00, 0x00,
        ],

        '+' => [
            0x00, 0x18, 0x18, 0x18,
            0x18, 0x7E, 0x7E, 0x18,
            0x18, 0x18, 0x18, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ],

        '=' => [
            0x00, 0x00, 0x00, 0x7E,
            0x00, 0x00, 0x7E, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ],

        '*' => [
            0x00, 0x18, 0x5A, 0x3C,
            0xFF, 0x3C, 0x5A, 0x18,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ],

        '/' => [
            0x06, 0x0C, 0x18, 0x30,
            0x60, 0xC0, 0x80, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ],

        '\\' => [
            0xC0, 0x60, 0x30, 0x18,
            0x0C, 0x06, 0x02, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ],

        '(' => [
            0x0C, 0x18, 0x30, 0x30,
            0x30, 0x30, 0x30, 0x30,
            0x30, 0x30, 0x18, 0x0C,
            0x00, 0x00, 0x00, 0x00,
        ],

        ')' => [
            0x30, 0x18, 0x0C, 0x0C,
            0x0C, 0x0C, 0x0C, 0x0C,
            0x0C, 0x0C, 0x18, 0x30,
            0x00, 0x00, 0x00, 0x00,
        ],

        '[' => [
            0x3C, 0x30, 0x30, 0x30,
            0x30, 0x30, 0x30, 0x30,
            0x30, 0x30, 0x30, 0x3C,
            0x00, 0x00, 0x00, 0x00,
        ],

        ']' => [
            0x3C, 0x0C, 0x0C, 0x0C,
            0x0C, 0x0C, 0x0C, 0x0C,
            0x0C, 0x0C, 0x0C, 0x3C,
            0x00, 0x00, 0x00, 0x00,
        ],

        '<' => [
            0x06, 0x0C, 0x18, 0x30,
            0x60, 0x30, 0x18, 0x0C,
            0x06, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ],

        '>' => [
            0x60, 0x30, 0x18, 0x0C,
            0x06, 0x0C, 0x18, 0x30,
            0x60, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ],

        '|' => [
            0x18, 0x18, 0x18, 0x18,
            0x18, 0x18, 0x18, 0x18,
            0x18, 0x18, 0x18, 0x18,
            0x18, 0x00, 0x00, 0x00,
        ],

        '"' => [
            0x66, 0x66, 0x66, 0x24,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ],

        '\'' => [
            0x18, 0x18, 0x18, 0x10,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ],

        '@' => [
            0x3C, 0x66, 0x6E, 0x76,
            0x76, 0x76, 0x7E, 0x60,
            0x60, 0x76, 0x66, 0x3C,
            0x00, 0x00, 0x00, 0x00,
        ],

        '#' => [
            0x36, 0x36, 0x7F, 0x36,
            0x36, 0x7F, 0x36, 0x36,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ],

        '$' => [
            0x18, 0x3E, 0x60, 0x3C,
            0x06, 0x06, 0x3C, 0x60,
            0x7C, 0x18, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ],

        '%' => [
            0x62, 0x66, 0x0C, 0x18,
            0x30, 0x60, 0x66, 0x46,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ],

        '&' => [
            0x1C, 0x36, 0x36, 0x1C,
            0x38, 0x6C, 0x66, 0x66,
            0x66, 0x6C, 0x38, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ],

        '^' => [
            0x18, 0x3C, 0x66, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ],

        // ====================================================================
        // UNSUPPORTED
        // ====================================================================

        _ => [
            0x7E, 0x42, 0x5A, 0x5A,
            0x5A, 0x42, 0x7E, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ],
    }
}