use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

static mut IDT: Option<InterruptDescriptorTable> = None;

extern "x86-interrupt" fn breakpoint_handler(
stack_frame: InterruptStackFrame,
) {
let _ = stack_frame;

loop {
    core::hint::spin_loop();
}

}

pub fn init() {
let mut idt = InterruptDescriptorTable::new();

idt.breakpoint.set_handler_fn(breakpoint_handler);

unsafe {
    IDT = Some(idt);

    if let Some(idt) = IDT.as_ref() {
        idt.load();
    }
}

}
