use crate::drivers::keyboard::ps2;

use pc_keyboard::{DecodedKey, HandleControl, Keyboard, ScancodeSet1, layouts};

use spin::Mutex;

const INPUT_BUFFER_SIZE: usize = 256;
const MAP_BUFFER_SIZE: usize = 64;

struct InputBuffer {
    data: [u8; INPUT_BUFFER_SIZE],
    read: usize,
    write: usize,
}

impl InputBuffer {
    const fn new() -> Self {
        Self {
            data: [0; INPUT_BUFFER_SIZE],
            read: 0,
            write: 0,
        }
    }

    fn push(&mut self, byte: u8) {
        let next = (self.write + 1) % INPUT_BUFFER_SIZE;

        if next == self.read {
            return;
        }

        self.data[self.write] = byte;
        self.write = next;
    }

    fn pop(&mut self) -> Option<u8> {
        if self.read == self.write {
            return None;
        }

        let byte = self.data[self.read];

        self.read = (self.read + 1) % INPUT_BUFFER_SIZE;

        Some(byte)
    }
}

struct MapBuffer {
    data: [u8; MAP_BUFFER_SIZE],
    len: usize,
}

impl MapBuffer {
    const fn new() -> Self {
        Self {
            data: [0; MAP_BUFFER_SIZE],
            len: 0,
        }
    }

    fn push(&mut self, byte: u8) {
        if self.len < MAP_BUFFER_SIZE {
            self.data[self.len] = byte;
            self.len += 1;
        }
    }

    fn pop(&mut self) -> Option<u8> {
        if self.len == 0 {
            return None;
        }

        let byte = self.data[0];

        let mut i = 1;

        while i < self.len {
            self.data[i - 1] = self.data[i];
            i += 1;
        }

        self.len -= 1;

        Some(byte)
    }
}

static KEYBOARD: Mutex<Option<Keyboard<layouts::Us104Key, ScancodeSet1>>> = Mutex::new(None);

static INPUT: Mutex<InputBuffer> = Mutex::new(InputBuffer::new());

static MAP_OUTPUT: Mutex<MapBuffer> = Mutex::new(MapBuffer::new());

pub fn init() {
    ps2::init();

    let keyboard = Keyboard::new(
        ScancodeSet1::new(),
        layouts::Us104Key,
        HandleControl::Ignore,
    );

    *KEYBOARD.lock() = Some(keyboard);
}

fn hex_digit(value: u8) -> u8 {
    match value {
        0..=9 => b'0' + value,
        _ => b'A' + (value - 10),
    }
}

fn push_hex(value: u8) {
    let mut output = MAP_OUTPUT.lock();

    output.push(b'0');
    output.push(b'x');
    output.push(hex_digit(value >> 4));
    output.push(hex_digit(value & 0x0F));
}

fn process_scancode(scancode: u8) {
    /*
     * Set 1 keyboard release codes normally have bit 7 set.
     *
     * Example:
     *
     * A press    = 0x1E
     * A release  = 0x9E
     *
     * We only record key presses.
     */
    if scancode & 0x80 != 0 {
        return;
    }

    /*
     * Mapping test output:
     *
     * SC:0x1E\n
     *
     * The main kernel will display this.
     */
    {
        let mut output = MAP_OUTPUT.lock();

        output.push(b'S');
        output.push(b'C');
        output.push(b':');

        output.push(b'0');
        output.push(b'x');

        output.push(hex_digit(scancode >> 4));
        output.push(hex_digit(scancode & 0x0F));

        output.push(b'\n');
    }

    /*
     * Normal keyboard decoder.
     */
    let mut keyboard_guard = KEYBOARD.lock();

    let Some(keyboard) = keyboard_guard.as_mut() else {
        return;
    };

    let Ok(Some(event)) = keyboard.add_byte(scancode) else {
        return;
    };

    let Some(key) = keyboard.process_keyevent(event) else {
        return;
    };

    match key {
        DecodedKey::Unicode(character) => {
            if character.is_ascii() {
                INPUT.lock().push(character as u8);
            }
        }

        DecodedKey::RawKey(_) => {}
    }
}

pub fn poll() {
    while let Some(scancode) = ps2::read_scancode() {
        process_scancode(scancode);
    }
}

pub fn read() -> Option<u8> {
    INPUT.lock().pop()
}

pub fn read_map_output() -> Option<u8> {
    MAP_OUTPUT.lock().pop()
}

pub fn handle_interrupt() {
    poll();
}
