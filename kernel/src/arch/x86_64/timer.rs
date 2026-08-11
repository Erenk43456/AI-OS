use core::sync::atomic::{AtomicU64, Ordering};
use core::arch::asm;

// ============================================================
// PIT
// ============================================================

const PIT_FREQUENCY: u32 = 1_193_182;

// 100 Hz = 10 ms
const TIMER_FREQUENCY: u32 = 100;

const PIT_COMMAND: u16 = 0x43;
const PIT_CHANNEL0: u16 = 0x40;

// ============================================================
// TICKS
// ============================================================

static TICKS: AtomicU64 = AtomicU64::new(0);

// ============================================================
// PORT IO
// ============================================================

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

// ============================================================
// INIT
// ============================================================

pub fn init() {
    let divisor = PIT_FREQUENCY / TIMER_FREQUENCY;

    // Channel 0
    // Access mode: low byte + high byte
    // Mode 3: square wave
    write_port(PIT_COMMAND, 0x36);

    // Low byte
    write_port(PIT_CHANNEL0, (divisor & 0xFF) as u8);

    // High byte
    write_port(PIT_CHANNEL0, ((divisor >> 8) & 0xFF) as u8);
}

// ============================================================
// INTERRUPT
// ============================================================

pub fn tick() {
    TICKS.fetch_add(1, Ordering::Relaxed);
}

// ============================================================
// CURRENT TICKS
// ============================================================

pub fn ticks() -> u64 {
    TICKS.load(Ordering::Relaxed)
}

// ============================================================
// TIME
// ============================================================

pub fn milliseconds() -> u64 {
    ticks() * 10
}