#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

mod arch;
mod console;
mod drivers;
mod input;
mod keyboard;

use bootloader_api::{
    entry_point,
    BootInfo,
};

use console::{
    Color,
    Console,
};

use core::panic::PanicInfo;

use input::event::InputEvent;
use input::queue;

entry_point!(kernel_main);

// ============================================================
// KERNEL
// ============================================================

fn kernel_main(
    boot_info: &'static mut BootInfo,
) -> ! {
    // ========================================================
    // INTERRUPTS
    // ========================================================

    arch::x86_64::interrupts::disable();

    // ========================================================
    // CPU / INTERRUPT INITIALIZATION
    // ========================================================

    arch::x86_64::gdt::init();

    arch::x86_64::idt::init();

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

    arch::x86_64::interrupts::unmask_timer();

    arch::x86_64::interrupts::unmask_keyboard();

    // ========================================================
    // FRAMEBUFFER
    // ========================================================

    if let Some(framebuffer) =
        boot_info.framebuffer.as_mut()
    {
        let info = framebuffer.info();

        let buffer =
            framebuffer.buffer_mut();

        let mut console =
            Console::new(buffer, info);

        console.clear();

        // ====================================================
        // AI-OS BOOT SPLASH
        // ====================================================

        console.println(
            "",
            Color::WHITE,
        );

        console.println(
            "",
            Color::WHITE,
        );

        console.println(
            "                 AI-OS",
            Color::BLUE,
        );

        console.println(
            "",
            Color::WHITE,
        );

        console.println(
            "        ARTIFICIAL INTELLIGENCE",
            Color::WHITE,
        );

        console.println(
            "          OPERATING SYSTEM",
            Color::WHITE,
        );

        console.println(
            "",
            Color::WHITE,
        );

        console.println(
            "",
            Color::WHITE,
        );

        console.println(
            "              INITIALIZING",
            Color::BLUE,
        );

        console.println(
            "",
            Color::WHITE,
        );

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

        console.println(
            "",
            Color::WHITE,
        );

        console.println(
            "",
            Color::WHITE,
        );

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

        console.println(
            "",
            Color::WHITE,
        );

        console.println(
            "AI-OS SYSTEM READY",
            Color::BLUE,
        );

        console.println(
            "Framebuffer:",
            Color::WHITE,
        );

        console.println(
            "Resolution will be shown below",
            Color::WHITE,
        );

        console.println(
            "",
            Color::WHITE,
        );

        console.println(
            "",
            Color::WHITE,
        );

        // ====================================================
        // SYSTEM INFORMATION
        // ====================================================

        console.println(
            "AI-OS",
            Color::BLUE,
        );

        console.println(
            "----------------------------------------",
            Color::BLUE,
        );

        console.println(
            "Kernel       : AI-OS Kernel",
            Color::WHITE,
        );

        console.println(
            "Architecture : x86_64",
            Color::WHITE,
        );

        console.println(
            "Input        : Turkish Q",
            Color::WHITE,
        );

        console.println(
            "Events       : InputEvent",
            Color::WHITE,
        );

        console.println(
            "Queue        : 128 events",
            Color::WHITE,
        );

        console.println(
            "----------------------------------------",
            Color::BLUE,
        );

        console.println(
            "",
            Color::WHITE,
        );

        // ====================================================
        // SHELL PROMPT
        // ====================================================

        console.print(
            "ai-os@system:~$ ",
            Color::WHITE,
        );

        // ====================================================
        // ENABLE INTERRUPTS
        // ====================================================

        arch::x86_64::interrupts::enable();

        // ====================================================
        // SHELL INPUT PROTECTION
        // ====================================================

        let mut shell_input_x =
            console.cursor_x();

        let mut shell_input_y =
            console.cursor_y();

        // ====================================================
        // CURSOR
        // ====================================================

        let mut cursor_visible =
            true;

        let mut last_cursor_tick =
            arch::x86_64::timer::ticks();

        console.toggle_cursor(true);

        // ====================================================
        // MAIN LOOP
        // ====================================================

        loop {
            // ==================================================
            // CURSOR BLINK
            // ==================================================

            let current_tick =
                arch::x86_64::timer::ticks();

            if current_tick
                .wrapping_sub(last_cursor_tick)
                >= 50
            {
                last_cursor_tick =
                    current_tick;

                cursor_visible =
                    !cursor_visible;

                console.toggle_cursor(
                    cursor_visible,
                );
            }

            // ==================================================
            // PROCESS INPUT EVENTS
            // ==================================================

            while let Some(event) =
                queue::pop()
            {
                console.toggle_cursor(false);

                match event {
                    // ==========================================
                    // CHARACTER
                    // ==========================================

                    InputEvent::KeyPress(
                        character,
                    ) => {
                        console.print_char(
                            character,
                            Color::WHITE,
                        );
                    }

                    // ==========================================
                    // BACKSPACE
                    // ==========================================

                    InputEvent::Backspace => {
                        let cursor_x =
                            console.cursor_x();

                        let cursor_y =
                            console.cursor_y();

                        if cursor_y
                            == shell_input_y
                            && cursor_x
                                > shell_input_x
                        {
                            console.backspace();
                        }
                    }

                    // ==========================================
                    // ENTER
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
                    // TAB
                    // ==========================================

                    InputEvent::Tab => {
                        console.print(
                            "    ",
                            Color::WHITE,
                        );
                    }

                    // ==========================================
                    // ARROW UP
                    // ==========================================

                    InputEvent::ArrowUp => {
                        // Shell history
                        // daha sonra eklenecek.
                    }

                    // ==========================================
                    // ARROW DOWN
                    // ==========================================

                    InputEvent::ArrowDown => {
                        // Shell history
                        // daha sonra eklenecek.
                    }

                    // ==========================================
                    // ARROW LEFT
                    // ==========================================

                    InputEvent::ArrowLeft => {
                        // Cursor navigation
                        // daha sonra eklenecek.
                    }

                    // ==========================================
                    // ARROW RIGHT
                    // ==========================================

                    InputEvent::ArrowRight => {
                        // Cursor navigation
                        // daha sonra eklenecek.
                    }
                }

                // ==============================================
                // RESTORE CURSOR
                // ==============================================

                cursor_visible = true;

                console.toggle_cursor(true);

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
fn panic(
    _info: &PanicInfo,
) -> ! {
    loop {
        core::hint::spin_loop();
    }
}