use spin::Once;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use super::interrupts::InterruptIndex;
use crate::keyboard;

static IDT: Once<InterruptDescriptorTable> = Once::new();

extern "x86-interrupt" fn breakpoint_handler(_stack_frame: InterruptStackFrame) {
    loop {
        core::hint::spin_loop();
    }
}

extern "x86-interrupt" fn keyboard_handler(_stack_frame: InterruptStackFrame) {
    keyboard::handle_interrupt();

    super::interrupts::end_of_interrupt(InterruptIndex::Keyboard);
}

pub fn init() {
    let idt = IDT.call_once(|| {
        let mut idt = InterruptDescriptorTable::new();

        idt.breakpoint.set_handler_fn(breakpoint_handler);

        idt[InterruptIndex::Keyboard.as_u8()].set_handler_fn(keyboard_handler);

        idt
    });

    idt.load();
}
