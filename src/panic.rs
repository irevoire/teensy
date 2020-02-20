//! # Panic
//! The macro in this file will generate a function with the `#[panic_handler]` needed to compile
//! your code.
//! You can choose a comportement in the following macros.
//!
//! If you want to create your own panic function you should use this signature:
//! ```rust
//! #[panic_handler]
//! fn panic(pi: &core::panic::PanicInfo) -> !;
//! ```
//!
//! If you want to include one of the already defined panic in your code use:
//! ```rust
//! teensy::define_panic!{empty}
//! ```

/// Helper macro to call the other panic macro
/// Use it like that:
/// ```rust
/// teensy::define_panic!{empty}
/// ```
#[macro_export(local_inner_macros)] // this way we are sure the macro are in scope
macro_rules! define_panic {
    (empty) => {
        empty_panic!();
    };
    (blink) => {
        blink_panic!();
    };
    (uart) => {
        uart_panic!();
    };
}

/// A panic doing nothing
#[macro_export]
macro_rules! empty_panic {
    () => {
        #[panic_handler]
        fn empty_panic(_pi: &core::panic::PanicInfo) -> ! {
            loop {}
        }
    };
}

/// A panic blinking a led
#[macro_export]
macro_rules! blink_panic {
    () => {
        #[panic_handler]
        fn blink_panic(_pi: &core::panic::PanicInfo) -> ! {
            // we don't know how was the crate included
            use teensy::*;
            // here we don't know if the port holding the led is on
            // so we need to reconfigure everything
            let (wdog, sim) = unsafe { (watchdog::Watchdog::new(), sim::Sim::new()) };
            wdog.disable();
            sim.enable_clock(sim::Clock::PortC);

            // now we can make our led blink
            let mut led = unsafe { make_pin!(led).make_gpio().with_output() };

            loop {
                led.toggle();
                sleep::sleep_ms(50);
            }
        }
    };
}

/// A panic sending your message on the uart serial port 1 in a loop
#[macro_export]
macro_rules! uart_panic {
    () => {
        #[panic_handler]
        fn uart_panic(pi: &core::panic::PanicInfo) -> ! {
            // we don't know how was the crate included
            use core::fmt::Write;
            use teensy::*;
            // here we don't know if the port holding the led is on
            // so we need to reconfigure everything
            let (wdog, sim) = unsafe { (watchdog::Watchdog::new(), sim::Sim::new()) };
            wdog.disable();
            sim.enable_clock(sim::Clock::PortB);

            let uart = unsafe {
                let mut uart = uart::UART::new(uart::UART0);
                uart.setup(sim, 115200);
                uart
            };

            writeln!(uart, "Teensy panicked").unwrap();
            loop {
                writeln!(uart, "{}", pi).unwrap();
                sleep::sleep_ms(500);
            }
        }
    };
}
