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
//! teensy::define_panic!(empty);
//! ```

/// Helper macro to call the other panic macro
/// Use it like that:
/// ```rust
/// teensy::define_panic!(empty);
/// ```
#[macro_export]
macro_rules! define_panic {
    (empty) => {
        empty_panic!();
    }
}

/// A panic doing nothing
#[macro_export]
macro_rules! empty_panic {
    () => {
#[panic_handler]
        fn empty_panic(_pi: &core::panic::PanicInfo) -> ! {
            loop {}
        }
    }
}

