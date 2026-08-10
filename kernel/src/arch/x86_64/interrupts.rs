pub fn init() {
// PIC ve IRQ altyapısı bir sonraki aşamada kurulacak.
}

pub fn enable() {
x86_64::instructions::interrupts::enable();
}
