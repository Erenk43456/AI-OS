warning: in the working copy of 'build.rs', LF will be replaced by CRLF the next time Git touches it
warning: in the working copy of 'kernel/src/arch/x86_64/gdt.rs', LF will be replaced by CRLF the next time Git touches it
warning: in the working copy of 'kernel/src/arch/x86_64/idt.rs', LF will be replaced by CRLF the next time Git touches it
warning: in the working copy of 'kernel/src/arch/x86_64/interrupts.rs', LF will be replaced by CRLF the next time Git touches it
warning: in the working copy of 'kernel/src/console/mod.rs', LF will be replaced by CRLF the next time Git touches it
warning: in the working copy of 'kernel/src/drivers/keyboard/ps2.rs', LF will be replaced by CRLF the next time Git touches it
warning: in the working copy of 'kernel/src/drivers/keyboard/scancode.rs', LF will be replaced by CRLF the next time Git touches it
warning: in the working copy of 'kernel/src/input/event.rs', LF will be replaced by CRLF the next time Git touches it
warning: in the working copy of 'kernel/src/input/queue.rs', LF will be replaced by CRLF the next time Git touches it
warning: in the working copy of 'kernel/src/keyboard.rs', LF will be replaced by CRLF the next time Git touches it
warning: in the working copy of 'kernel/src/main.rs', LF will be replaced by CRLF the next time Git touches it
warning: in the working copy of 'src/main.rs', LF will be replaced by CRLF the next time Git touches it
[1mdiff --git a/build.rs b/build.rs[m
[1mindex 40a82e7..b4afd1d 100644[m
[1m--- a/build.rs[m
[1m+++ b/build.rs[m
[36m@@ -2,23 +2,19 @@[m [muse std::env;[m
 use std::path::PathBuf;[m
 [m
 fn main() {[m
[31m-let kernel_path = env::var_os("CARGO_BIN_FILE_KERNEL")[m
[31m-.expect("Kernel binary bulunamadı.");[m
[32m+[m[32m    let kernel_path = env::var_os("CARGO_BIN_FILE_KERNEL").expect("Kernel binary bulunamadı.");[m
 [m
[31m-let kernel_path = PathBuf::from(kernel_path);[m
[32m+[m[32m    let kernel_path = PathBuf::from(kernel_path);[m
 [m
[31m-let out_dir = PathBuf::from([m
[31m-    env::var_os("OUT_DIR").expect("OUT_DIR bulunamadı.")[m
[31m-);[m
[32m+[m[32m    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR bulunamadı."));[m
 [m
[31m-let uefi_image = out_dir.join("ai-os-uefi.img");[m
[32m+[m[32m    let uefi_image = out_dir.join("ai-os-uefi.img");[m
 [m
[31m-println!("cargo:rerun-if-changed=kernel/src");[m
[32m+[m[32m    println!("cargo:rerun-if-changed=kernel/src");[m
 [m
[31m-bootloader::UefiBoot::new(&kernel_path)[m
[31m-    .create_disk_image(&uefi_image)[m
[31m-    .expect("UEFI disk image oluşturulamadı.");[m
[32m+[m[32m    bootloader::UefiBoot::new(&kernel_path)[m
[32m+[m[32m        .create_disk_image(&uefi_image)[m
[32m+[m[32m        .expect("UEFI disk image oluşturulamadı.");[m
 [m
[31m-println!("cargo:rustc-env=UEFI_PATH={}", uefi_image.display());[m
[31m-[m
[31m-}[m
\ No newline at end of file[m
[32m+[m[32m    println!("cargo:rustc-env=UEFI_PATH={}", uefi_image.display());[m
[32m+[m[32m}[m
[1mdiff --git a/kernel/src/arch/x86_64/gdt.rs b/kernel/src/arch/x86_64/gdt.rs[m
[1mindex 570ee27..cf7f14d 100644[m
[1m--- a/kernel/src/arch/x86_64/gdt.rs[m
[1m+++ b/kernel/src/arch/x86_64/gdt.rs[m
[36m@@ -1,3 +1,3 @@[m
 pub fn init() {[m
[31m-// GDT altyapısı bir sonraki aşamada kurulacak.[m
[32m+[m[32m    // GDT altyapısı bir sonraki aşamada kurulacak.[m
 }[m
[1mdiff --git a/kernel/src/arch/x86_64/idt.rs b/kernel/src/arch/x86_64/idt.rs[m
[1mindex 4483805..d5ad6b5 100644[m
[1m--- a/kernel/src/arch/x86_64/idt.rs[m
[1m+++ b/kernel/src/arch/x86_64/idt.rs[m
[36m@@ -1,29 +1,34 @@[m
[32m+[m[32muse spin::Once;[m
 use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};[m
 [m
[31m-static mut IDT: Option<InterruptDescriptorTable> = None;[m
[32m+[m[32muse super::interrupts::InterruptIndex;[m
 [m
[31m-extern "x86-interrupt" fn breakpoint_handler([m
[31m-stack_frame: InterruptStackFrame,[m
[31m-) {[m
[31m-let _ = stack_frame;[m
[32m+[m[32mstatic IDT: Once<InterruptDescriptorTable> = Once::new();[m
 [m
[31m-loop {[m
[31m-    core::hint::spin_loop();[m
[32m+[m[32mextern "x86-interrupt" fn breakpoint_handler(_stack_frame: InterruptStackFrame) {[m
[32m+[m[32m    loop {[m
[32m+[m[32m        core::hint::spin_loop();[m
[32m+[m[32m    }[m
 }[m
 [m
[32m+[m[32mextern "x86-interrupt" fn keyboard_handler(_stack_frame: InterruptStackFrame) {[m
[32m+[m[32m    crate::keyboard::handle_interrupt();[m
[32m+[m
[32m+[m[32m    super::interrupts::end_of_interrupt(InterruptIndex::Keyboard);[m
 }[m
 [m
 pub fn init() {[m
[31m-let mut idt = InterruptDescriptorTable::new();[m
[32m+[m[32m    let idt = IDT.call_once(|| {[m
[32m+[m[32m        let mut idt = InterruptDescriptorTable::new();[m
[32m+[m
[32m+[m[32m        idt.breakpoint.set_handler_fn(breakpoint_handler);[m
 [m
[31m-idt.breakpoint.set_handler_fn(breakpoint_handler);[m
[32m+[m[32m        idt[InterruptIndex::Keyboard.as_u8()].set_handler_fn(keyboard_handler);[m
 [m
[31m-unsafe {[m
[31m-    IDT = Some(idt);[m
[32m+[m[32m        idt[m
[32m+[m[32m    });[m
 [m
[31m-    if let Some(idt) = IDT.as_ref() {[m
[32m+[m[32m    unsafe {[m
         idt.load();[m
     }[m
 }[m
[31m-[m
[31m-}[m
[1mdiff --git a/kernel/src/arch/x86_64/interrupts.rs b/kernel/src/arch/x86_64/interrupts.rs[m
[1mindex a8760f4..07c7c7a 100644[m
[1m--- a/kernel/src/arch/x86_64/interrupts.rs[m
[1m+++ b/kernel/src/arch/x86_64/interrupts.rs[m
[36m@@ -1,7 +1,125 @@[m
[32m+[m[32muse core::arch::asm;[m
[32m+[m
[32m+[m[32m#[repr(u8)][m
[32m+[m[32m#[derive(Clone, Copy)][m
[32m+[m[32mpub enum InterruptIndex {[m
[32m+[m[32m    Timer = 32,[m
[32m+[m[32m    Keyboard = 33,[m
[32m+[m[32m}[m
[32m+[m
[32m+[m[32mimpl InterruptIndex {[m
[32m+[m[32m    pub const fn as_u8(self) -> u8 {[m
[32m+[m[32m        self as u8[m
[32m+[m[32m    }[m
[32m+[m[32m}[m
[32m+[m
[32m+[m[32mconst PIC1_COMMAND: u16 = 0x20;[m
[32m+[m[32mconst PIC1_DATA: u16 = 0x21;[m
[32m+[m
[32m+[m[32mconst PIC2_COMMAND: u16 = 0xA0;[m
[32m+[m[32mconst PIC2_DATA: u16 = 0xA1;[m
[32m+[m
[32m+[m[32mconst ICW1_INIT: u8 = 0x10;[m
[32m+[m[32mconst ICW1_ICW4: u8 = 0x01;[m
[32m+[m[32mconst ICW4_8086: u8 = 0x01;[m
[32m+[m
[32m+[m[32mconst PIC_EOI: u8 = 0x20;[m
[32m+[m
[32m+[m[32mfn io_wait() {[m
[32m+[m[32m    unsafe {[m
[32m+[m[32m        asm!([m
[32m+[m[32m            "out 0x80, al",[m
[32m+[m[32m            in("al") 0u8,[m
[32m+[m[32m            options(nomem, nostack, preserves_flags)[m
[32m+[m[32m        );[m
[32m+[m[32m    }[m
[32m+[m[32m}[m
[32m+[m
[32m+[m[32mfn write_port(port: u16, value: u8) {[m
[32m+[m[32m    unsafe {[m
[32m+[m[32m        asm!([m
[32m+[m[32m            "out dx, al",[m
[32m+[m[32m            in("dx") port,[m
[32m+[m[32m            in("al") value,[m
[32m+[m[32m            options(nomem, nostack, preserves_flags)[m
[32m+[m[32m        );[m
[32m+[m[32m    }[m
[32m+[m[32m}[m
[32m+[m
[32m+[m[32mfn read_port(port: u16) -> u8 {[m
[32m+[m[32m    let value: u8;[m
[32m+[m
[32m+[m[32m    unsafe {[m
[32m+[m[32m        asm!([m
[32m+[m[32m            "in al, dx",[m
[32m+[m[32m            in("dx") port,[m
[32m+[m[32m            out("al") value,[m
[32m+[m[32m            options(nomem, nostack, preserves_flags)[m
[32m+[m[32m        );[m
[32m+[m[32m    }[m
[32m+[m
[32m+[m[32m    value[m
[32m+[m[32m}[m
[32m+[m
 pub fn init() {[m
[31m-// PIC ve IRQ altyapısı bir sonraki aşamada kurulacak.[m
[32m+[m[32m    let master_mask = read_port(PIC1_DATA);[m
[32m+[m[32m    let slave_mask = read_port(PIC2_DATA);[m
[32m+[m
[32m+[m[32m    // Initialize master PIC.[m
[32m+[m[32m    write_port(PIC1_COMMAND, ICW1_INIT | ICW1_ICW4);[m
[32m+[m[32m    io_wait();[m
[32m+[m
[32m+[m[32m    // Initialize slave PIC.[m
[32m+[m[32m    write_port(PIC2_COMMAND, ICW1_INIT | ICW1_ICW4);[m
[32m+[m[32m    io_wait();[m
[32m+[m
[32m+[m[32m    // Remap master PIC to interrupts 32-39.[m
[32m+[m[32m    write_port(PIC1_DATA, 32);[m
[32m+[m[32m    io_wait();[m
[32m+[m
[32m+[m[32m    // Remap slave PIC to interrupts 40-47.[m
[32m+[m[32m    write_port(PIC2_DATA, 40);[m
[32m+[m[32m    io_wait();[m
[32m+[m
[32m+[m[32m    // Tell master that slave is connected to IRQ2.[m
[32m+[m[32m    write_port(PIC1_DATA, 4);[m
[32m+[m[32m    io_wait();[m
[32m+[m
[32m+[m[32m    // Tell slave its cascade identity.[m
[32m+[m[32m    write_port(PIC2_DATA, 2);[m
[32m+[m[32m    io_wait();[m
[32m+[m
[32m+[m[32m    // Set 8086 mode.[m
[32m+[m[32m    write_port(PIC1_DATA, ICW4_8086);[m
[32m+[m[32m    io_wait();[m
[32m+[m
[32m+[m[32m    write_port(PIC2_DATA, ICW4_8086);[m
[32m+[m[32m    io_wait();[m
[32m+[m
[32m+[m[32m    // Restore interrupt masks.[m
[32m+[m[32m    write_port(PIC1_DATA, master_mask);[m
[32m+[m[32m    write_port(PIC2_DATA, slave_mask);[m
[32m+[m[32m}[m
[32m+[m
[32m+[m[32mpub fn end_of_interrupt(interrupt: InterruptIndex) {[m
[32m+[m[32m    if interrupt.as_u8() >= 40 {[m
[32m+[m[32m        write_port(PIC2_COMMAND, PIC_EOI);[m
[32m+[m[32m    }[m
[32m+[m
[32m+[m[32m    write_port(PIC1_COMMAND, PIC_EOI);[m
[32m+[m[32m}[m
[32m+[m
[32m+[m[32mpub fn unmask_keyboard() {[m
[32m+[m[32m    let mask = read_port(PIC1_DATA);[m
[32m+[m
[32m+[m[32m    // IRQ1 = PS/2 keyboard.[m
[32m+[m[32m    write_port(PIC1_DATA, mask & !(1 << 1));[m
 }[m
 [m
 pub fn enable() {[m
[31m-x86_64::instructions::interrupts::enable();[m
[32m+[m[32m    x86_64::instructions::interrupts::enable();[m
[32m+[m[32m}[m
[32m+[m
[32m+[m[32mpub fn disable() {[m
[32m+[m[32m    x86_64::instructions::interrupts::disable();[m
 }[m
[1mdiff --git a/kernel/src/console/mod.rs b/kernel/src/console/mod.rs[m
[1mindex 30b1e2e..13a76c4 100644[m
[1m--- a/kernel/src/console/mod.rs[m
[1m+++ b/kernel/src/console/mod.rs[m
[36m@@ -16,11 +16,7 @@[m [mimpl Color {[m
         b: 255,[m
     };[m
 [m
[31m-    pub const GREEN: Color = Color {[m
[31m-        r: 0,[m
[31m-        g: 255,[m
[31m-        b: 0,[m
[31m-    };[m
[32m+[m[32m    pub const GREEN: Color = Color { r: 0, g: 255, b: 0 };[m
 [m
     pub const BLUE: Color = Color {[m
         r: 80,[m
[36m@@ -43,10 +39,7 @@[m [mpub struct Console<'a> {[m
 }[m
 [m
 impl<'a> Console<'a> {[m
[31m-    pub fn new([m
[31m-        buffer: &'a mut [u8],[m
[31m-        info: FrameBufferInfo,[m
[31m-    ) -> Self {[m
[32m+[m[32m    pub fn new(buffer: &'a mut [u8], info: FrameBufferInfo) -> Self {[m
         let mut console = Self {[m
             buffer,[m
             info,[m
[36m@@ -72,8 +65,7 @@[m [mimpl<'a> Console<'a> {[m
             return;[m
         }[m
 [m
[31m-        let offset =[m
[31m-            (y * self.info.stride + x) * self.info.bytes_per_pixel;[m
[32m+[m[32m        let offset = (y * self.info.stride + x) * self.info.bytes_per_pixel;[m
 [m
         match self.info.pixel_format {[m
             PixelFormat::Rgb => {[m
[36m@@ -99,179 +91,67 @@[m [mimpl<'a> Console<'a> {[m
     fn glyph(c: u8) -> [u8; 8] {[m
         match c {[m
             b'A' => [[m
[31m-                0b01110,[m
[31m-                0b10001,[m
[31m-                0b10001,[m
[31m-                0b11111,[m
[31m-                0b10001,[m
[31m-                0b10001,[m
[31m-                0b10001,[m
[31m-                0,[m
[32m+[m[32m                0b01110, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001, 0,[m
             ],[m
 [m
             b'I' => [[m
[31m-                0b11111,[m
[31m-                0b00100,[m
[31m-                0b00100,[m
[31m-                0b00100,[m
[31m-                0b00100,[m
[31m-                0b00100,[m
[31m-                0b11111,[m
[31m-                0,[m
[32m+[m[32m                0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b11111, 0,[m
             ],[m
 [m
             b'O' => [[m
[31m-                0b01110,[m
[31m-                0b10001,[m
[31m-                0b10001,[m
[31m-                0b10001,[m
[31m-                0b10001,[m
[31m-                0b10001,[m
[31m-                0b01110,[m
[31m-                0,[m
[32m+[m[32m                0b01110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110, 0,[m
             ],[m
 [m
             b'S' => [[m
[31m-                0b01111,[m
[31m-                0b10000,[m
[31m-                0b10000,[m
[31m-                0b01110,[m
[31m-                0b00001,[m
[31m-                0b00001,[m
[31m-                0b11110,[m
[31m-                0,[m
[32m+[m[32m                0b01111, 0b10000, 0b10000, 0b01110, 0b00001, 0b00001, 0b11110, 0,[m
             ],[m
 [m
             b'K' => [[m
[31m-                0b10001,[m
[31m-                0b10010,[m
[31m-                0b10100,[m
[31m-                0b11000,[m
[31m-                0b10100,[m
[31m-                0b10010,[m
[31m-                0b10001,[m
[31m-                0,[m
[32m+[m[32m                0b10001, 0b10010, 0b10100, 0b11000, 0b10100, 0b10010, 0b10001, 0,[m
             ],[m
 [m
             b'E' => [[m
[31m-                0b11111,[m
[31m-                0b10000,[m
[31m-                0b10000,[m
[31m-                0b11110,[m
[31m-                0b10000,[m
[31m-                0b10000,[m
[31m-                0b11111,[m
[31m-                0,[m
[32m+[m[32m                0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b11111, 0,[m
             ],[m
 [m
             b'R' => [[m
[31m-                0b11110,[m
[31m-                0b10001,[m
[31m-                0b10001,[m
[31m-                0b11110,[m
[31m-                0b10100,[m
[31m-                0b10010,[m
[31m-                0b10001,[m
[31m-                0,[m
[32m+[m[32m                0b11110, 0b10001, 0b10001, 0b11110, 0b10100, 0b10010, 0b10001, 0,[m
             ],[m
 [m
             b'N' => [[m
[31m-                0b10001,[m
[31m-                0b11001,[m
[31m-                0b10101,[m
[31m-                0b10011,[m
[31m-                0b10001,[m
[31m-                0b10001,[m
[31m-                0b10001,[m
[31m-                0,[m
[32m+[m[32m                0b10001, 0b11001, 0b10101, 0b10011, 0b10001, 0b10001, 0b10001, 0,[m
             ],[m
 [m
             b'L' => [[m
[31m-                0b10000,[m
[31m-                0b10000,[m
[31m-                0b10000,[m
[31m-                0b10000,[m
[31m-                0b10000,[m
[31m-                0b10000,[m
[31m-                0b11111,[m
[31m-                0,[m
[32m+[m[32m                0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b11111, 0,[m
             ],[m
 [m
             b'T' => [[m
[31m-                0b11111,[m
[31m-                0b00100,[m
[31m-                0b00100,[m
[31m-                0b00100,[m
[31m-                0b00100,[m
[31m-                0b00100,[m
[31m-                0b00100,[m
[31m-                0,[m
[32m+[m[32m                0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0,[m
             ],[m
 [m
             b'B' => [[m
[31m-                0b11110,[m
[31m-                0b10001,[m
[31m-                0b10001,[m
[31m-                0b11110,[m
[31m-                0b10001,[m
[31m-                0b10001,[m
[31m-                0b11110,[m
[31m-                0,[m
[32m+[m[32m                0b11110, 0b10001, 0b10001, 0b11110, 0b10001, 0b10001, 0b11110, 0,[m
             ],[m
 [m
             b'D' => [[m
[31m-                0b11110,[m
[31m-                0b10001,[m
[31m-                0b10001,[m
[31m-                0b10001,[m
[31m-                0b10001,[m
[31m-                0b10001,[m
[31m-                0b11110,[m
[31m-                0,[m
[32m+[m[32m                0b11110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b11110, 0,[m
             ],[m
 [m
             b'F' => [[m
[31m-                0b11111,[m
[31m-                0b10000,[m
[31m-                0b10000,[m
[31m-                0b11110,[m
[31m-                0b10000,[m
[31m-                0b10000,[m
[31m-                0b10000,[m
[31m-                0,[m
[32m+[m[32m                0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b10000, 0,[m
             ],[m
 [m
             b'M' => [[m
[31m-                0b10001,[m
[31m-                0b11011,[m
[31m-                0b10101,[m
[31m-                0b10101,[m
[31m-                0b10001,[m
[31m-                0b10001,[m
[31m-                0b10001,[m
[31m-                0,[m
[32m+[m[32m                0b10001, 0b11011, 0b10101, 0b10101, 0b10001, 0b10001, 0b10001, 0,[m
             ],[m
 [m
             b'[' => [[m
[31m-                0b11111,[m
[31m-                0b10000,[m
[31m-                0b10000,[m
[31m-                0b10000,[m
[31m-                0b10000,[m
[31m-                0b10000,[m
[31m-                0b11111,[m
[31m-                0,[m
[32m+[m[32m                0b11111, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b11111, 0,[m
             ],[m
 [m
             b']' => [[m
[31m-                0b11111,[m
[31m-                0b00001,[m
[31m-                0b00001,[m
[31m-                0b00001,[m
[31m-                0b00001,[m
[31m-                0b00001,[m
[31m-                0b11111,[m
[31m-                0,[m
[32m+[m[32m                0b11111, 0b00001, 0b00001, 0b00001, 0b00001, 0b00001, 0b11111, 0,[m
             ],[m
 [m
             b' ' => [0; 8],[m
[36m@@ -280,23 +160,13 @@[m [mimpl<'a> Console<'a> {[m
         }[m
     }[m
 [m
[31m-    fn draw_char([m
[31m-        &mut self,[m
[31m-        x: usize,[m
[31m-        y: usize,[m
[31m-        c: u8,[m
[31m-        color: Color,[m
[31m-    ) {[m
[32m+[m[32m    fn draw_char(&mut self, x: usize, y: usize, c: u8, color: Color) {[m
         let glyph = Self::glyph(c);[m
 [m
         for (row, bits) in glyph.iter().enumerate() {[m
             for col in 0..5 {[m
                 if bits & (1 << (4 - col)) != 0 {[m
[31m-                    self.draw_pixel([m
[31m-                        x + col,[m
[31m-                        y + row,[m
[31m-                        color,[m
[31m-                    );[m
[32m+[m[32m                    self.draw_pixel(x + col, y + row, color);[m
                 }[m
             }[m
         }[m
[36m@@ -310,12 +180,7 @@[m [mimpl<'a> Console<'a> {[m
                 continue;[m
             }[m
 [m
[31m-            self.draw_char([m
[31m-                self.cursor_x,[m
[31m-                self.cursor_y,[m
[31m-                byte,[m
[31m-                color,[m
[31m-            );[m
[32m+[m[32m            self.draw_char(self.cursor_x, self.cursor_y, byte, color);[m
 [m
             self.cursor_x += 6;[m
 [m
[36m@@ -332,4 +197,4 @@[m [mimpl<'a> Console<'a> {[m
         self.cursor_x = 10;[m
         self.cursor_y += 10;[m
     }[m
[31m-}[m
\ No newline at end of file[m
[32m+[m[32m}[m
[1mdiff --git a/kernel/src/drivers/keyboard/ps2.rs b/kernel/src/drivers/keyboard/ps2.rs[m
[1mindex e69de29..23e9638 100644[m
[1m--- a/kernel/src/drivers/keyboard/ps2.rs[m
[1m+++ b/kernel/src/drivers/keyboard/ps2.rs[m
[36m@@ -0,0 +1,42 @@[m
[32m+[m[32muse core::arch::asm;[m
[32m+[m
[32m+[m[32mconst DATA_PORT: u16 = 0x60;[m
[32m+[m[32mconst STATUS_PORT: u16 = 0x64;[m
[32m+[m
[32m+[m[32mpub fn init() {[m
[32m+[m[32m    // PS/2 controller initialization will be expanded here.[m
[32m+[m[32m}[m
[32m+[m
[32m+[m[32mpub fn can_read() -> bool {[m
[32m+[m[32m    let status: u8;[m
[32m+[m
[32m+[m[32m    unsafe {[m
[32m+[m[32m        asm!([m
[32m+[m[32m            "in al, dx",[m
[32m+[m[32m            in("dx") STATUS_PORT,[m
[32m+[m[32m            out("al") status,[m
[32m+[m[32m            options(nomem, nostack, preserves_flags)[m
[32m+[m[32m        );[m
[32m+[m[32m    }[m
[32m+[m
[32m+[m[32m    status & 1 != 0[m
[32m+[m[32m}[m
[32m+[m
[32m+[m[32mpub fn read_scancode() -> Option<u8> {[m
[32m+[m[32m    if !can_read() {[m
[32m+[m[32m        return None;[m
[32m+[m[32m    }[m
[32m+[m
[32m+[m[32m    let scancode: u8;[m
[32m+[m
[32m+[m[32m    unsafe {[m
[32m+[m[32m        asm!([m
[32m+[m[32m            "in al, dx",[m
[32m+[m[32m            in("dx") DATA_PORT,[m
[32m+[m[32m            out("al") scancode,[m
[32m+[m[32m            options(nomem, nostack, preserves_flags)[m
[32m+[m[32m        );[m
[32m+[m[32m    }[m
[32m+[m
[32m+[m[32m    Some(scancode)[m
[32m+[m[32m}[m
[1mdiff --git a/kernel/src/drivers/keyboard/scancode.rs b/kernel/src/drivers/keyboard/scancode.rs[m
[1mindex e69de29..e35f685 100644[m
[1m--- a/kernel/src/drivers/keyboard/scancode.rs[m
[1m+++ b/kernel/src/drivers/keyboard/scancode.rs[m
[36m@@ -0,0 +1,7 @@[m
[32m+[m[32mpub fn is_key_release(scancode: u8) -> bool {[m
[32m+[m[32m    scancode & 0x80 != 0[m
[32m+[m[32m}[m
[32m+[m
[32m+[m[32mpub fn strip_release_bit(scancode: u8) -> u8 {[m
[32m+[m[32m    scancode & 0x7F[m
[32m+[m[32m}[m
[1mdiff --git a/kernel/src/input/event.rs b/kernel/src/input/event.rs[m
[1mindex e69de29..3f62931 100644[m
[1m--- a/kernel/src/input/event.rs[m
[1m+++ b/kernel/src/input/event.rs[m
[36m@@ -0,0 +1,4 @@[m
[32m+[m[32m#[derive(Clone, Copy, Debug)][m
[32m+[m[32mpub enum InputEvent {[m
[32m+[m[32m    KeyPress(u8),[m
[32m+[m[32m}[m
[1mdiff --git a/kernel/src/input/queue.rs b/kernel/src/input/queue.rs[m
[1mindex e69de29..e83e60e 100644[m
[1m--- a/kernel/src/input/queue.rs[m
[1m+++ b/kernel/src/input/queue.rs[m
[36m@@ -0,0 +1,55 @@[m
[32m+[m[32muse spin::Mutex;[m
[32m+[m
[32m+[m[32muse super::event::InputEvent;[m
[32m+[m
[32m+[m[32mconst QUEUE_SIZE: usize = 128;[m
[32m+[m
[32m+[m[32mstruct InputQueue {[m
[32m+[m[32m    buffer: [Option<InputEvent>; QUEUE_SIZE],[m
[32m+[m[32m    read: usize,[m
[32m+[m[32m    write: usize,[m
[32m+[m[32m}[m
[32m+[m
[32m+[m[32mimpl InputQueue {[m
[32m+[m[32m    const fn new() -> Self {[m
[32m+[m[32m        Self {[m
[32m+[m[32m            buffer: [None; QUEUE_SIZE],[m
[32m+[m[32m            read: 0,[m
[32m+[m[32m            write: 0,[m
[32m+[m[32m        }[m
[32m+[m[32m    }[m
[32m+[m
[32m+[m[32m    fn push(&mut self, event: InputEvent) {[m
[32m+[m[32m        let next = (self.write + 1) % QUEUE_SIZE;[m
[32m+[m
[32m+[m[32m        // Queue full: drop the newest event.[m
[32m+[m[32m        if next == self.read {[m
[32m+[m[32m            return;[m
[32m+[m[32m        }[m
[32m+[m
[32m+[m[32m        self.buffer[self.write] = Some(event);[m
[32m+[m[32m        self.write = next;[m
[32m+[m[32m    }[m
[32m+[m
[32m+[m[32m    fn pop(&mut self) -> Option<InputEvent> {[m
[32m+[m[32m        if self.read == self.write {[m
[32m+[m[32m            return None;[m
[32m+[m[32m        }[m
[32m+[m
[32m+[m[32m        let event = self.buffer[self.read].take();[m
[32m+[m
[32m+[m[32m        self.read = (self.read + 1) % QUEUE_SIZE;[m
[32m+[m
[32m+[m[32m        event[m
[32m+[m[32m    }[m
[32m+[m[32m}[m
[32m+[m
[32m+[m[32mstatic QUEUE: Mutex<InputQueue> = Mutex::new(InputQueue::new());[m
[32m+[m
[32m+[m[32mpub fn push(event: InputEvent) {[m
[32m+[m[32m    QUEUE.lock().push(event);[m
[32m+[m[32m}[m
[32m+[m
[32m+[m[32mpub fn pop() -> Option<InputEvent> {[m
[32m+[m[32m    QUEUE.lock().pop()[m
[32m+[m[32m}[m
[1mdiff --git a/kernel/src/keyboard.rs b/kernel/src/keyboard.rs[m
[1mindex 05fce0f..58197e6 100644[m
[1m--- a/kernel/src/keyboard.rs[m
[1m+++ b/kernel/src/keyboard.rs[m
[36m@@ -1,93 +1,58 @@[m
[31m-use core::arch::asm;[m
[31m-use core::cell::UnsafeCell;[m
[32m+[m[32muse pc_keyboard::{DecodedKey, HandleControl, Keyboard, ScancodeSet1, layouts};[m
 [m
[31m-use pc_keyboard::{[m
[31m-layouts,[m
[31m-DecodedKey,[m
[31m-HandleControl,[m
[31m-Keyboard,[m
[31m-ScancodeSet1,[m
[31m-};[m
[32m+[m[32muse crate::drivers::keyboard::ps2;[m
[32m+[m[32muse crate::input::{event::InputEvent, queue};[m
 [m
[31m-struct KeyboardState {[m
[31m-inner: UnsafeCell<Option<Keyboard<layouts::Us104Key, ScancodeSet1>>>,[m
[31m-}[m
[31m-[m
[31m-unsafe impl Sync for KeyboardState {}[m
[32m+[m[32muse spin::Mutex;[m
 [m
[31m-static KEYBOARD: KeyboardState = KeyboardState {[m
[31m-inner: UnsafeCell::new(None),[m
[31m-};[m
[32m+[m[32mstatic KEYBOARD: Mutex<Option<Keyboard<layouts::Us104Key, ScancodeSet1>>> = Mutex::new(None);[m
 [m
 pub fn init() {[m
[31m-unsafe {[m
[31m-*KEYBOARD.inner.get() = Some(Keyboard::new([m
[31m-ScancodeSet1::new(),[m
[31m-layouts::Us104Key,[m
[31m-HandleControl::Ignore,[m
[31m-));[m
[31m-}[m
[31m-}[m
[31m-[m
[31m-fn keyboard_available() -> bool {[m
[31m-let status: u8;[m
[31m-[m
[31m-unsafe {[m
[31m-    asm!([m
[31m-        "in al, dx",[m
[31m-        in("dx") 0x64u16,[m
[31m-        out("al") status,[m
[31m-        options(nomem, nostack, preserves_flags)[m
[31m-    );[m
[31m-}[m
[31m-[m
[31m-status & 1 != 0[m
[31m-[m
[31m-}[m
[32m+[m[32m    ps2::init();[m
 [m
[31m-fn read_scancode() -> u8 {[m
[31m-let scancode: u8;[m
[32m+[m[32m    let mut keyboard = KEYBOARD.lock();[m
 [m
[31m-unsafe {[m
[31m-    asm!([m
[31m-        "in al, dx",[m
[31m-        in("dx") 0x60u16,[m
[31m-        out("al") scancode,[m
[31m-        options(nomem, nostack, preserves_flags)[m
[31m-    );[m
[32m+[m[32m    *keyboard = Some(Keyboard::new([m
[32m+[m[32m        ScancodeSet1::new(),[m
[32m+[m[32m        layouts::Us104Key,[m
[32m+[m[32m        HandleControl::Ignore,[m
[32m+[m[32m    ));[m
 }[m
 [m
[31m-scancode[m
[31m-[m
[31m-}[m
[32m+[m[32mpub fn handle_interrupt() {[m
[32m+[m[32m    let Some(scancode) = ps2::read_scancode() else {[m
[32m+[m[32m        return;[m
[32m+[m[32m    };[m
 [m
[31m-pub fn read_char() -> Option<u8> {[m
[31m-if !keyboard_available() {[m
[31m-return None;[m
[31m-}[m
[32m+[m[32m    queue::push(crate::input::event::InputEvent::KeyPress(scancode));[m
 [m
[31m-let scancode = read_scancode();[m
[32m+[m[32m    let mut keyboard = KEYBOARD.lock();[m
 [m
[31m-unsafe {[m
[31m-    let keyboard = &mut *KEYBOARD.inner.get();[m
[32m+[m[32m    let Some(keyboard) = keyboard.as_mut() else {[m
[32m+[m[32m        return;[m
[32m+[m[32m    };[m
 [m
[31m-    let keyboard = keyboard.as_mut()?;[m
[32m+[m[32m    let Ok(Some(key_event)) = keyboard.add_byte(scancode) else {[m
[32m+[m[32m        return;[m
[32m+[m[32m    };[m
 [m
[31m-    let key_event = keyboard.add_byte(scancode).ok()??;[m
[31m-[m
[31m-    let decoded_key = keyboard.process_keyevent(key_event)?;[m
[32m+[m[32m    let Some(decoded_key) = keyboard.process_keyevent(key_event) else {[m
[32m+[m[32m        return;[m
[32m+[m[32m    };[m
 [m
     match decoded_key {[m
         DecodedKey::Unicode(character) => {[m
             if character.is_ascii() {[m
[31m-                Some(character as u8)[m
[31m-            } else {[m
[31m-                None[m
[32m+[m[32m                queue::push(InputEvent::KeyPress(character as u8));[m
             }[m
         }[m
 [m
[31m-        DecodedKey::RawKey(_) => None,[m
[32m+[m[32m        DecodedKey::RawKey(_) => {}[m
     }[m
 }[m
 [m
[31m-}[m
\ No newline at end of file[m
[32m+[m[32mpub fn read_char() -> Option<u8> {[m
[32m+[m[32m    match queue::pop()? {[m
[32m+[m[32m        InputEvent::KeyPress(character) => Some(character),[m
[32m+[m[32m    }[m
[32m+[m[32m}[m
[1mdiff --git a/kernel/src/main.rs b/kernel/src/main.rs[m
[1mindex 2f4f4d8..0fb9161 100644[m
[1m--- a/kernel/src/main.rs[m
[1m+++ b/kernel/src/main.rs[m
[36m@@ -1,55 +1,80 @@[m
[32m+[m[32m#![feature(abi_x86_interrupt)][m
 #![no_std][m
 #![no_main][m
 [m
 mod arch;[m
 mod console;[m
[32m+[m[32mmod drivers;[m
[32m+[m[32mmod input;[m
 mod keyboard;[m
 [m
[31m-use bootloader_api::{entry_point, BootInfo};[m
[32m+[m[32muse bootloader_api::{BootInfo, entry_point};[m
 use console::{Color, Console};[m
 use core::panic::PanicInfo;[m
 [m
 entry_point!(kernel_main);[m
 [m
 fn kernel_main(boot_info: &'static mut BootInfo) -> ! {[m
[31m-// CPU interrupt descriptor table.[m
[31m-arch::x86_64::idt::init();[m
[32m+[m[32m    // Interrupts kapalıyken donanımı hazırla.[m
[32m+[m[32m    arch::x86_64::interrupts::disable();[m
 [m
[31m-if let Some(framebuffer) = boot_info.framebuffer.as_mut() {[m
[31m-    let info = framebuffer.info();[m
[31m-    let buffer = framebuffer.buffer_mut();[m
[32m+[m[32m    // Keyboard driver.[m
[32m+[m[32m    keyboard::init();[m
 [m
[31m-    let mut console = Console::new(buffer, info);[m
[32m+[m[32m    // Programmable Interrupt Controller.[m
[32m+[m[32m    arch::x86_64::interrupts::init();[m
 [m
[31m-    console.println(b"AI-OS KERNEL", Color::WHITE);[m
[31m-    console.println(b"", Color::WHITE);[m
[32m+[m[32m    // Interrupt Descriptor Table.[m
[32m+[m[32m    arch::x86_64::idt::init();[m
 [m
[31m-    console.println(b"[OK] BOOTLOADER", Color::GREEN);[m
[31m-    console.println(b"[OK] FRAMEBUFFER", Color::GREEN);[m
[31m-    console.println(b"[OK] KERNEL", Color::GREEN);[m
[31m-    console.println(b"[OK] IDT", Color::GREEN);[m
[32m+[m[32m    // Sadece keyboard IRQ1'i aç.[m
[32m+[m[32m    arch::x86_64::interrupts::unmask_keyboard();[m
 [m
[31m-    console.println(b"", Color::WHITE);[m
[32m+[m[32m    // CPU interruptlarını aktif et.[m
[32m+[m[32m    arch::x86_64::interrupts::enable();[m
 [m
[31m-    console.println(b"AI-OS READY", Color::WHITE);[m
[31m-    console.println(b"", Color::WHITE);[m
[32m+[m[32m    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {[m
[32m+[m[32m        let info = framebuffer.info();[m
[32m+[m[32m        let buffer = framebuffer.buffer_mut();[m
 [m
[31m-    console.print(b"ai-os> ", Color::WHITE);[m
[32m+[m[32m        let mut console = Console::new(buffer, info);[m
[32m+[m
[32m+[m[32m        console.println(b"AI-OS KERNEL", Color::WHITE);[m
[32m+[m[32m        console.println(b"", Color::WHITE);[m
[32m+[m
[32m+[m[32m        console.println(b"[OK] BOOTLOADER", Color::GREEN);[m
[32m+[m[32m        console.println(b"[OK] FRAMEBUFFER", Color::GREEN);[m
[32m+[m[32m        console.println(b"[OK] KERNEL", Color::GREEN);[m
[32m+[m[32m        console.println(b"[OK] PIC", Color::GREEN);[m
[32m+[m[32m        console.println(b"[OK] IDT", Color::GREEN);[m
[32m+[m[32m        console.println(b"[OK] PS/2", Color::GREEN);[m
[32m+[m[32m        console.println(b"[OK] KEYBOARD", Color::GREEN);[m
[32m+[m[32m        console.println(b"[OK] INPUT", Color::GREEN);[m
[32m+[m
[32m+[m[32m        console.println(b"", Color::WHITE);[m
[32m+[m
[32m+[m[32m        console.println(b"AI-OS READY", Color::WHITE);[m
[32m+[m[32m        console.println(b"", Color::WHITE);[m
[32m+[m
[32m+[m[32m        console.print(b"ai-os> ", Color::WHITE);[m
[32m+[m
[32m+[m[32m        loop {[m
[32m+[m[32m            if let Some(character) = keyboard::read_char() {[m
[32m+[m[32m                console.print(&[character], Color::WHITE);[m
[32m+[m[32m            }[m
[32m+[m
[32m+[m[32m            core::hint::spin_loop();[m
[32m+[m[32m        }[m
[32m+[m[32m    }[m
 [m
     loop {[m
         core::hint::spin_loop();[m
     }[m
 }[m
 [m
[31m-loop {[m
[31m-    core::hint::spin_loop();[m
[31m-}[m
[31m-[m
[31m-}[m
[31m-[m
 #[panic_handler][m
 fn panic(_info: &PanicInfo) -> ! {[m
[31m-loop {[m
[31m-core::hint::spin_loop();[m
[31m-}[m
[32m+[m[32m    loop {[m
[32m+[m[32m        core::hint::spin_loop();[m
[32m+[m[32m    }[m
 }[m
[1mdiff --git a/src/main.rs b/src/main.rs[m
[1mindex 8a3354b..e5078ce 100644[m
[1m--- a/src/main.rs[m
[1m+++ b/src/main.rs[m
[36m@@ -3,45 +3,37 @@[m [muse std::path::PathBuf;[m
 use std::process::Command;[m
 [m
 fn main() {[m
[31m-println!();[m
[31m-println!("=================================");[m
[31m-println!("        AI-OS QEMU Launcher");[m
[31m-println!("=================================");[m
[31m-println!();[m
[31m-[m
[31m-let uefi_path = PathBuf::from([m
[31m-    env::var("UEFI_PATH")[m
[31m-        .expect("UEFI_PATH bulunamadı. Önce cargo build çalıştırın.")[m
[31m-);[m
[31m-[m
[31m-println!("[OK] UEFI image bulundu:");[m
[31m-println!("{}", uefi_path.display());[m
[31m-println!();[m
[31m-[m
[31m-let qemu = r"C:\Program Files\qemu\qemu-system-x86_64.exe";[m
[31m-let firmware = r"C:\Program Files\qemu\share\edk2-x86_64-code.fd";[m
[31m-[m
[31m-println!("[OK] QEMU başlatılıyor...");[m
[31m-println!();[m
[31m-[m
[31m-let status = Command::new(qemu)[m
[31m-    .args([[m
[31m-        "-drive",[m
[31m-        &format!([m
[31m-            "if=pflash,format=raw,readonly=on,file={}",[m
[31m-            firmware[m
[31m-        ),[m
[31m-        "-drive",[m
[31m-        &format!([m
[31m-            "format=raw,file={}",[m
[31m-            uefi_path.display()[m
[31m-        ),[m
[31m-    ])[m
[31m-    .status()[m
[31m-    .expect("QEMU başlatılamadı.");[m
[31m-[m
[31m-if !status.success() {[m
[31m-    eprintln!("QEMU hata koduyla kapandı: {:?}", status.code());[m
[32m+[m[32m    println!();[m
[32m+[m[32m    println!("=================================");[m
[32m+[m[32m    println!("        AI-OS QEMU Launcher");[m
[32m+[m[32m    println!("=================================");[m
[32m+[m[32m    println!();[m
[32m+[m
[32m+[m[32m    let uefi_path = PathBuf::from([m
[32m+[m[32m        env::var("UEFI_PATH").expect("UEFI_PATH bulunamadı. Önce cargo build çalıştırın."),[m
[32m+[m[32m    );[m
[32m+[m
[32m+[m[32m    println!("[OK] UEFI image bulundu:");[m
[32m+[m[32m    println!("{}", uefi_path.display());[m
[32m+[m[32m    println!();[m
[32m+[m
[32m+[m[32m    let qemu = r"C:\Program Files\qemu\qemu-system-x86_64.exe";[m
[32m+[m[32m    let firmware = r"C:\Program Files\qemu\share\edk2-x86_64-code.fd";[m
[32m+[m
[32m+[m[32m    println!("[OK] QEMU başlatılıyor...");[m
[32m+[m[32m    println!();[m
[32m+[m
[32m+[m[32m    let status = Command::new(qemu)[m
[32m+[m[32m        .args([[m
[32m+[m[32m            "-drive",[m
[32m+[m[32m            &format!("if=pflash,format=raw,readonly=on,file={}", firmware),[m
[32m+[m[32m            "-drive",[m
[32m+[m[32m            &format!("format=raw,file={}", uefi_path.display()),[m
[32m+[m[32m        ])[m
[32m+[m[32m        .status()[m
[32m+[m[32m        .expect("QEMU başlatılamadı.");[m
[32m+[m
[32m+[m[32m    if !status.success() {[m
[32m+[m[32m        eprintln!("QEMU hata koduyla kapandı: {:?}", status.code());[m
[32m+[m[32m    }[m
 }[m
[31m-[m
[31m-}[m
\ No newline at end of file[m
