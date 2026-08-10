#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

mod arch;
mod console;
mod drivers;
mod input;
mod keyboard;
mod keyboard_layout;

use bootloader_api::{BootInfo, entry_point};

use console::{Color, Console};

use core::panic::PanicInfo;

entry_point!(kernel_main);

// ============================================================
// KERNEL
// ============================================================

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    // ========================================================
    // INTERRUPTS
    // ========================================================

    arch::x86_64::interrupts::disable();

    // ========================================================
    // KEYBOARD
    // ========================================================

    keyboard::init();

    // ========================================================
    // FRAMEBUFFER
    // ========================================================

    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
        let info = framebuffer.info();

        let buffer = framebuffer.buffer_mut();

        let mut console = Console::new(buffer, info);

        // ====================================================
        // BOOT
        // ====================================================

        console.println("AI-OS KERNEL", Color::WHITE);

        console.println("", Color::WHITE);

        console.println("[OK] BOOTLOADER", Color::GREEN);

        console.println("[OK] FRAMEBUFFER", Color::GREEN);

        console.println("[OK] KERNEL", Color::GREEN);

        console.println("[OK] PS2", Color::GREEN);

        console.println("[OK] KEYBOARD", Color::GREEN);

        console.println("[OK] TURKISH Q", Color::GREEN);

        console.println("", Color::WHITE);

        console.println("AI-OS READY", Color::WHITE);

        console.println("", Color::WHITE);

        console.print("ai-os> ", Color::WHITE);

        // ====================================================
        // MAIN LOOP
        // ====================================================

        loop {
            keyboard::poll();

            while let Some(character) = keyboard::read() {
                match character {
                    // ========================================
                    // ENTER
                    // ========================================
                    b'\n' | b'\r' => {
                        console.println("", Color::WHITE);

                        console.print("ai-os> ", Color::WHITE);
                    }

                    // ========================================
                    // BACKSPACE
                    // ========================================
                    0x08 => {
                        console.backspace();
                    }

                    // ========================================
                    // TAB
                    // ========================================
                    b'\t' => {
                        console.print("    ", Color::WHITE);
                    }

                    // ==================================================
                    // TÜRKÇE KÜÇÜK
                    // ==================================================
                    0x80 => {
                        console.print_char('ğ', Color::WHITE);
                    }

                    0x81 => {
                        console.print_char('ü', Color::WHITE);
                    }

                    0x82 => {
                        console.print_char('ş', Color::WHITE);
                    }

                    0x83 => {
                        console.print_char('i', Color::WHITE);
                    }

                    0x84 => {
                        console.print_char('ö', Color::WHITE);
                    }

                    0x85 => {
                        console.print_char('ç', Color::WHITE);
                    }

                    // ==================================================
                    // TÜRKÇE BÜYÜK
                    // ==================================================
                    0x86 => {
                        console.print_char('Ğ', Color::WHITE);
                    }

                    0x87 => {
                        console.print_char('Ü', Color::WHITE);
                    }

                    0x88 => {
                        console.print_char('Ş', Color::WHITE);
                    }

                    0x89 => {
                        console.print_char('İ', Color::WHITE);
                    }

                    0x8A => {
                        console.print_char('Ö', Color::WHITE);
                    }

                    0x8B => {
                        console.print_char('Ç', Color::WHITE);
                    }

                    // ==================================================
                    // NOKTASIZ I
                    // ==================================================
                    0x8C => {
                        console.print_char('ı', Color::WHITE);
                    }

                    0x8D => {
                        console.print_char('I', Color::WHITE);
                    }

                    // ========================================
                    // NORMAL ASCII
                    // ========================================
                    character if character.is_ascii() => {
                        console.print_char(character as char, Color::WHITE);
                    }

                    // ========================================
                    // UNKNOWN
                    // ========================================
                    _ => {}
                }
            }

            core::hint::spin_loop();
        }
    }

    // ========================================================
    // NO FRAMEBUFFER
    // ========================================================

    loop {
        core::hint::spin_loop();
    }
}

// ============================================================
// PANIC
// ============================================================

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
