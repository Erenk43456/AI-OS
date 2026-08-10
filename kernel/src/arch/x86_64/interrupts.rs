use core::arch::asm;

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum InterruptIndex {
    Timer = 32,
    Keyboard = 33,
}

impl InterruptIndex {
    pub const fn as_u8(self) -> u8 {
        self as u8
    }
}

const PIC1_COMMAND: u16 = 0x20;
const PIC1_DATA: u16 = 0x21;

const PIC2_COMMAND: u16 = 0xA0;
const PIC2_DATA: u16 = 0xA1;

const ICW1_INIT: u8 = 0x10;
const ICW1_ICW4: u8 = 0x01;
const ICW4_8086: u8 = 0x01;

const PIC_EOI: u8 = 0x20;

fn io_wait() {
    unsafe {
        asm!(
            "out 0x80, al",
            in("al") 0u8,
            options(nomem, nostack, preserves_flags)
        );
    }
}

fn write_port(port: u16, value: u8) {
    unsafe {
        asm!(
            "out dx, al",
            in("dx") port,
            in("al") value,
            options(nomem, nostack, preserves_flags)
        );
    }
}

fn read_port(port: u16) -> u8 {
    let value: u8;

    unsafe {
        asm!(
            "in al, dx",
            in("dx") port,
            out("al") value,
            options(nomem, nostack, preserves_flags)
        );
    }

    value
}

pub fn init() {
    let master_mask = read_port(PIC1_DATA);
    let slave_mask = read_port(PIC2_DATA);

    // Initialize master PIC.
    write_port(PIC1_COMMAND, ICW1_INIT | ICW1_ICW4);
    io_wait();

    // Initialize slave PIC.
    write_port(PIC2_COMMAND, ICW1_INIT | ICW1_ICW4);
    io_wait();

    // Remap master PIC to interrupts 32-39.
    write_port(PIC1_DATA, 32);
    io_wait();

    // Remap slave PIC to interrupts 40-47.
    write_port(PIC2_DATA, 40);
    io_wait();

    // Tell master that slave is connected to IRQ2.
    write_port(PIC1_DATA, 4);
    io_wait();

    // Tell slave its cascade identity.
    write_port(PIC2_DATA, 2);
    io_wait();

    // Set 8086 mode.
    write_port(PIC1_DATA, ICW4_8086);
    io_wait();

    write_port(PIC2_DATA, ICW4_8086);
    io_wait();

    // Restore interrupt masks.
    write_port(PIC1_DATA, master_mask);
    write_port(PIC2_DATA, slave_mask);
}

pub fn end_of_interrupt(interrupt: InterruptIndex) {
    if interrupt.as_u8() >= 40 {
        write_port(PIC2_COMMAND, PIC_EOI);
    }

    write_port(PIC1_COMMAND, PIC_EOI);
}

pub fn unmask_keyboard() {
    let mask = read_port(PIC1_DATA);

    // IRQ1 = PS/2 keyboard.
    write_port(PIC1_DATA, mask & !(1 << 1));
}

pub fn enable() {
    x86_64::instructions::interrupts::enable();
}

pub fn disable() {
    x86_64::instructions::interrupts::disable();
}
