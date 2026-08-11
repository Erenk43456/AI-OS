use spin::Once;
use x86_64::structures::idt::{
    InterruptDescriptorTable,
    InterruptStackFrame,
};

use super::interrupts::InterruptIndex;
use crate::keyboard;
use crate::arch::x86_64::timer;

static IDT: Once<InterruptDescriptorTable> = Once::new();

// ============================================================
// BREAKPOINT
// ============================================================

extern "x86-interrupt" fn breakpoint_handler(
    _stack_frame: InterruptStackFrame,
) {
    loop {
        core::hint::spin_loop();
    }
}

// ============================================================
// TIMER
// ============================================================

extern "x86-interrupt" fn timer_handler(
    _stack_frame: InterruptStackFrame,
) {
    timer::tick();

    super::interrupts::end_of_interrupt(
        InterruptIndex::Timer,
    );
}

// ============================================================
// KEYBOARD
// ============================================================

extern "x86-interrupt" fn keyboard_handler(
    _stack_frame: InterruptStackFrame,
) {
    keyboard::handle_interrupt();

    super::interrupts::end_of_interrupt(
        InterruptIndex::Keyboard,
    );
}

// ============================================================
// INIT
// ============================================================

pub fn init() {
    let idt = IDT.call_once(|| {
        let mut idt = InterruptDescriptorTable::new();

        idt.breakpoint.set_handler_fn(
            breakpoint_handler,
        );

        idt[InterruptIndex::Timer.as_u8()]
            .set_handler_fn(timer_handler);

        idt[InterruptIndex::Keyboard.as_u8()]
            .set_handler_fn(keyboard_handler);

        idt
    });

    idt.load();
}