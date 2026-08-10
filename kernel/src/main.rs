#![no_std]
#![no_main]

mod arch;
mod console;
mod keyboard;

use bootloader_api::{entry_point, BootInfo};
use console::{Color, Console};
use core::panic::PanicInfo;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
// CPU interrupt descriptor table.
arch::x86_64::idt::init();

if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
    let info = framebuffer.info();
    let buffer = framebuffer.buffer_mut();

    let mut console = Console::new(buffer, info);

    console.println(b"AI-OS KERNEL", Color::WHITE);
    console.println(b"", Color::WHITE);

    console.println(b"[OK] BOOTLOADER", Color::GREEN);
    console.println(b"[OK] FRAMEBUFFER", Color::GREEN);
    console.println(b"[OK] KERNEL", Color::GREEN);
    console.println(b"[OK] IDT", Color::GREEN);

    console.println(b"", Color::WHITE);

    console.println(b"AI-OS READY", Color::WHITE);
    console.println(b"", Color::WHITE);

    console.print(b"ai-os> ", Color::WHITE);

    loop {
        core::hint::spin_loop();
    }
}

loop {
    core::hint::spin_loop();
}

}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
loop {
core::hint::spin_loop();
}
}
