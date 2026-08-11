#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

mod arch;
mod console;
mod drivers;
mod input;
mod keyboard;
mod keyboard_layout;

use bootloader_api::{entry_point, BootInfo};

use console::{Color, Console};

use core::panic::PanicInfo;

use input::event::InputEvent;
use input::queue;

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
    // CPU / INTERRUPT INITIALIZATION
    // ========================================================

    // GDT
    arch::x86_64::gdt::init();

    // IDT
    arch::x86_64::idt::init();

    // PIC
    arch::x86_64::interrupts::init();

    // ========================================================
    // TIMER
    // ========================================================

    arch::x86_64::timer::init();

    // ========================================================
    // KEYBOARD
    // ========================================================

    keyboard::init();

    // ========================================================
    // IRQ UNMASK
    // ========================================================

    // PIT IRQ0
    arch::x86_64::interrupts::unmask_timer();

    // PS/2 Keyboard IRQ1
    arch::x86_64::interrupts::unmask_keyboard();

    // ========================================================
    // FRAMEBUFFER
    // ========================================================

    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
        let info = framebuffer.info();

        let buffer = framebuffer.buffer_mut();

        let mut console = Console::new(buffer, info);

        console.clear();

        // ====================================================
        // AI-OS BOOT SPLASH
        // ====================================================

        console.println("", Color::WHITE);
        console.println("", Color::WHITE);

        console.println(
            "                 AI-OS",
            Color::BLUE,
        );

        console.println("", Color::WHITE);

        console.println(
            "        ARTIFICIAL INTELLIGENCE",
            Color::WHITE,
        );

        console.println(
            "          OPERATING SYSTEM",
            Color::WHITE,
        );

        console.println("", Color::WHITE);
        console.println("", Color::WHITE);

        console.println(
            "              INITIALIZING",
            Color::BLUE,
        );

        console.println("", Color::WHITE);

        console.println(
            "                    .",
            Color::WHITE,
        );

        console.println(
            "                   . .",
            Color::WHITE,
        );

        console.println(
            "                  . . .",
            Color::WHITE,
        );

        console.println("", Color::WHITE);
        console.println("", Color::WHITE);

        // ====================================================
        // SYSTEM INITIALIZATION
        // ====================================================

        console.println(
            "AI-OS SYSTEM",
            Color::BLUE,
        );

        console.println(
            "----------------------------------------",
            Color::WHITE,
        );

        console.println(
            "Kernel              [ OK ]",
            Color::GREEN,
        );

        console.println(
            "Framebuffer         [ OK ]",
            Color::GREEN,
        );

        console.println(
            "Memory              [ OK ]",
            Color::GREEN,
        );

        console.println(
            "PS/2 Controller     [ OK ]",
            Color::GREEN,
        );

        console.println(
            "Keyboard            [ OK ]",
            Color::GREEN,
        );

        console.println(
            "Turkish Q Layout    [ OK ]",
            Color::GREEN,
        );

        console.println(
            "Input System        [ OK ]",
            Color::GREEN,
        );

        console.println(
            "Input Queue         [ OK ]",
            Color::GREEN,
        );

        console.println(
            "----------------------------------------",
            Color::WHITE,
        );

        console.println("", Color::WHITE);

        console.println(
            "AI-OS SYSTEM READY",
            Color::BLUE,
        );

        console.println("", Color::WHITE);
        console.println("", Color::WHITE);

        // ====================================================
        // SYSTEM INFORMATION
        // ====================================================

        console.println(
            "┌────────────────AI-OS────────────────────┐",
            Color::BLUE,
        );

        console.println(
            "│                                            │",
            Color::BLUE,
        );

        console.println(
            "│  Kernel      : AI-OS Kernel                │",
            Color::WHITE,
        );

        console.println(
            "│  Architecture: x86_64                      │",
            Color::WHITE,
        );

        console.println(
            "│  Input       : Turkish Q                   │",
            Color::WHITE,
        );

        console.println(
            "│  Events      : InputEvent                  │",
            Color::WHITE,
        );

        console.println(
            "│  Queue       : 128 events                  │",
            Color::WHITE,
        );

        console.println(
            "│                                            │",
            Color::BLUE,
        );

        console.println(
            "└────────────────────────────────────────────┘",
            Color::BLUE,
        );

        console.println("", Color::WHITE);

        // ====================================================
        // SHELL PROMPT
        // ====================================================

        console.print(
            "ai-os@system:~$ ",
            Color::WHITE,
        );

        arch::x86_64::interrupts::enable();

        // ====================================================
        // SHELL INPUT PROTECTION
        // ====================================================

        // Kullanıcının yazabileceği alanın başlangıç noktası.
        //
        // Prompt'un kendisi hiçbir zaman silinemez.

        let mut shell_input_x = console.cursor_x();
        let mut shell_input_y = console.cursor_y();

        // ====================================================
        // CURSOR
        // ====================================================

        let mut cursor_visible = true;
        let mut last_cursor_tick = arch::x86_64::timer::ticks();

        console.toggle_cursor(true);

        // ====================================================
        // MAIN LOOP
        // ====================================================

        loop {
            
            // ==================================================
            // CURSOR BLINK
            // ==================================================

            let current_tick = arch::x86_64::timer::ticks();

            if current_tick.wrapping_sub(last_cursor_tick) >= 50 {
                last_cursor_tick = current_tick;

                cursor_visible = !cursor_visible;

                console.toggle_cursor(cursor_visible);
            }

            // ==================================================
            // PROCESS INPUT EVENTS
            // ==================================================

            while let Some(event) = queue::pop() {
                console.toggle_cursor(false);

                match event {
                    // ==========================================
                    // KEY PRESS
                    // ==========================================

                    InputEvent::KeyPress(character) => {
                        match character {
                            // ==================================
                            // ENTER
                            // ==================================

                            b'\n' | b'\r' => {
                                console.println(
                                    "",
                                    Color::WHITE,
                                );

                                console.print(
                                    "ai-os@system:~$ ",
                                    Color::WHITE,
                                );

                                shell_input_x =
                                    console.cursor_x();

                                shell_input_y =
                                    console.cursor_y();
                            }

                            // ==================================
                            // BACKSPACE
                            // ==================================

                            0x08 => {
                                let cursor_x =
                                    console.cursor_x();

                                let cursor_y =
                                    console.cursor_y();

                                if cursor_y == shell_input_y
                                    && cursor_x > shell_input_x
                                {
                                    console.backspace();
                                }
                            }

                            // ==================================
                            // TAB
                            // ==================================

                            b'\t' => {
                                console.print(
                                    "    ",
                                    Color::WHITE,
                                );
                            }

                            // ==================================
                            // TÜRKÇE KÜÇÜK
                            // ==================================

                            0x80 => {
                                console.print_char(
                                    'ğ',
                                    Color::WHITE,
                                );
                            }

                            0x81 => {
                                console.print_char(
                                    'ü',
                                    Color::WHITE,
                                );
                            }

                            0x82 => {
                                console.print_char(
                                    'ş',
                                    Color::WHITE,
                                );
                            }

                            0x83 => {
                                console.print_char(
                                    'i',
                                    Color::WHITE,
                                );
                            }

                            0x84 => {
                                console.print_char(
                                    'ö',
                                    Color::WHITE,
                                );
                            }

                            0x85 => {
                                console.print_char(
                                    'ç',
                                    Color::WHITE,
                                );
                            }

                            // ==================================
                            // TÜRKÇE BÜYÜK
                            // ==================================

                            0x86 => {
                                console.print_char(
                                    'Ğ',
                                    Color::WHITE,
                                );
                            }

                            0x87 => {
                                console.print_char(
                                    'Ü',
                                    Color::WHITE,
                                );
                            }

                            0x88 => {
                                console.print_char(
                                    'Ş',
                                    Color::WHITE,
                                );
                            }

                            0x89 => {
                                console.print_char(
                                    'İ',
                                    Color::WHITE,
                                );
                            }

                            0x8A => {
                                console.print_char(
                                    'Ö',
                                    Color::WHITE,
                                );
                            }

                            0x8B => {
                                console.print_char(
                                    'Ç',
                                    Color::WHITE,
                                );
                            }

                            // ==================================
                            // NOKTASIZ I
                            // ==================================

                            0x8C => {
                                console.print_char(
                                    'ı',
                                    Color::WHITE,
                                );
                            }

                            0x8D => {
                                console.print_char(
                                    'I',
                                    Color::WHITE,
                                );
                            }

                            // ==================================
                            // NORMAL ASCII
                            // ==================================

                            character if character.is_ascii() => {
                                console.print_char(
                                    character as char,
                                    Color::WHITE,
                                );
                            }

                            // ==================================
                            // UNKNOWN
                            // ==================================

                            _ => {}
                        }
                    }

                    // ==========================================
                    // ARROW UP
                    // ==========================================

                    InputEvent::ArrowUp => {
                        // Şimdilik history sistemi yok.
                    }

                    // ==========================================
                    // ARROW DOWN
                    // ==========================================

                    InputEvent::ArrowDown => {
                        // Şimdilik history sistemi yok.
                    }

                    // ==========================================
                    // ARROW LEFT
                    // ==========================================

                    InputEvent::ArrowLeft => {
                        // Şimdilik cursor navigation yok.
                    }

                    // ==========================================
                    // ARROW RIGHT
                    // ==========================================

                    InputEvent::ArrowRight => {
                        // Şimdilik cursor navigation yok.
                    }

                    // ==========================================
                    // BACKSPACE EVENT
                    // ==========================================

                    InputEvent::Backspace => {
                        let cursor_x =
                            console.cursor_x();

                        let cursor_y =
                            console.cursor_y();

                        if cursor_y == shell_input_y
                            && cursor_x > shell_input_x
                        {
                            console.backspace();
                        }
                    }

                    // ==========================================
                    // ENTER EVENT
                    // ==========================================

                    InputEvent::Enter => {
                        console.println(
                            "",
                            Color::WHITE,
                        );

                        console.print(
                            "ai-os@system:~$ ",
                            Color::WHITE,
                        );

                        shell_input_x =
                            console.cursor_x();

                        shell_input_y =
                            console.cursor_y();
                    }

                    // ==========================================
                    // TAB EVENT
                    // ==========================================

                    InputEvent::Tab => {
                        console.print(
                            "    ",
                            Color::WHITE,
                        );
                    }
                }
                console.toggle_cursor(true);
                
                cursor_visible = true;
                last_cursor_tick =
                    arch::x86_64::timer::ticks();
            }

            // ==================================================
            // CPU HINT
            // ==================================================

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