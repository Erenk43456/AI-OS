use spin::Once;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

static IDT: Once<InterruptDescriptorTable> = Once::new();

extern "x86-interrupt" fn breakpoint_handler(_stack_frame: InterruptStackFrame) {
    loop {
        core::hint::spin_loop();
    }
}

pub fn init() {
    let idt = IDT.call_once(|| {
        let mut idt = InterruptDescriptorTable::new();

        idt.breakpoint.set_handler_fn(breakpoint_handler);

        idt
    });

    idt.load();
}
