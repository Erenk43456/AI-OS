#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

mod arch;
mod console;
mod drivers;
mod input;
mod keyboard;

use bootloader_api::{BootInfo, entry_point};
use console::{Color, Console};
use core::panic::PanicInfo;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    // Interrupts kapalıyken donanımı hazırla.
    arch::x86_64::interrupts::disable();

    // Keyboard driver.
    keyboard::init();

    // Programmable Interrupt Controller.
    arch::x86_64::interrupts::init();

    // Interrupt Descriptor Table.
    arch::x86_64::idt::init();

    // Sadece keyboard IRQ1'i aç.
    arch::x86_64::interrupts::unmask_keyboard();

    // CPU interruptlarını aktif et.
    arch::x86_64::interrupts::enable();

    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
        let info = framebuffer.info();
        let buffer = framebuffer.buffer_mut();

        let mut console = Console::new(buffer, info);

        console.println(b"AI-OS KERNEL", Color::WHITE);
        console.println(b"", Color::WHITE);

        console.println(b"[OK] BOOTLOADER", Color::GREEN);
        console.println(b"[OK] FRAMEBUFFER", Color::GREEN);
        console.println(b"[OK] KERNEL", Color::GREEN);
        console.println(b"[OK] PIC", Color::GREEN);
        console.println(b"[OK] IDT", Color::GREEN);
        console.println(b"[OK] PS/2", Color::GREEN);
        console.println(b"[OK] KEYBOARD", Color::GREEN);
        console.println(b"[OK] INPUT", Color::GREEN);

        console.println(b"", Color::WHITE);

        console.println(b"AI-OS READY", Color::WHITE);
        console.println(b"", Color::WHITE);

        console.print(b"ai-os> ", Color::WHITE);

        loop {
            let status = keyboard::status();

            let hex = b"0123456789ABCDEF";

            console.print(b" [", Color::WHITE);
            console.print(&[hex[(status >> 4) as usize]], Color::WHITE);
            console.print(&[hex[(status & 0x0F) as usize]], Color::WHITE);
            console.print(b"]", Color::WHITE);

            for _ in 0..5_000_000 {
                core::hint::spin_loop();
            }
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
