#![allow(dead_code)]
#![feature(stdsimd)]
#![no_std]

//! # Teensy
//! This crate provide multiple helpers functions to manipulate the teensy in baremetal rust.
//! Currently everything is tested on a teensy 2.3. It provide:
//! * A bootloader
//! * Helper to handle the gpio pins
//!
//! # Quick start
//! ## Add the crate as a dependency
//! Currently the crate is still in heavy development so there is no release on crate.io.
//! To include this crate in your project add this line to the dependency section of your
//! `Cargo.toml` file:
//! ```toml
//! teensy = { git = "https://github.com/irevoire/teensy.git"}
//! ```
//!
//! ## Compile with this crate
//! ### Linker
//! To compile code for the teensy this crate provide multiple section that you'll need to put in
//! the right place. To do this we provide a linker script that you can copy paste as `layout.ld`:
//! ```ld
//! MEMORY
//! {
//! 	FLASH (rx) : ORIGIN = 0x00000000, LENGTH = 256K
//! 	RAM  (rwx) : ORIGIN = 0x1FFF8000, LENGTH = 64K
//! }
//!
//! SECTIONS
//! {
//! 	.text : {
//! 		. = 0;
//! 		KEEP(*(.vectors))
//! 		. = 0x400;
//! 		KEEP(*(.flashconfig*))
//! 		. = ALIGN(4);
//! 		*(.text*)
//!     } > FLASH = 0xFF
//!
//!     .rodata : {
//! 	    *(.rodata*)
//!     } > FLASH
//!
//!     _stack_top = ORIGIN(RAM) + LENGTH(RAM);
//!
//!     /DISCARD/ : {
//! 	    *(.ARM.*)
//!     }
//! }
//! ```
//! For more information on this script you can check the original script !(here)[layout.ld].
//!
//! ### Target
//! In order to compile your code for the teensy you need to specify the architecture you are
//! targetting. Create a `.cargo/config` file like that:
//! ```toml
//! [build]
//! target = "thumbv7em-none-eabi"
//!
//! [target.thumbv7em-none-eabi]
//! rustflags = [
//! 	"-C", "link-arg=-Tlayout.ld",
//! ]
//! ```
//!
//! ### Nightly rust
//! Finally to compile with this crate you need to use the nightly channel, just run:
//! ```bash
//! % rustup override set nightly
//! ```
//!
//! ## Code with this crate
//! Here is a minimal example of valid code:
//! ```rust
//! #![no_std]
//! #![no_main]
//!
//!#[panic_handler]
//! fn teensy_panic(_pi: &core::panic::PanicInfo) -> ! {
//!     loop {}
//! }
//!
//! #[no_mangle]
//! fn main() {
//!     loop {}
//! }
//! ```
//!
//! ## Flashing the Teensy
//! We provide a Makefile to generate a binary for the teensy 2.3 and sending it using the
//! `teensy_loader_cli` command:
//! ```Makefile
//! BIN=my_application
//! OUTDIR=target/thumbv7em-none-eabi/release
//! HEX=$(OUTDIR)/$(BIN).hex
//! ELF=$(OUTDIR)/$(BIN)
//!
//! all:: $(ELF)
//!
//! .PHONY: $(ELF)
//! $(ELF):
//! 	cargo build --release
//!
//! $(HEX): $(ELF)
//! 	arm-none-eabi-objcopy -O ihex $(ELF) $(HEX)
//!
//! .PHONY: flash
//! flash: $(HEX)
//! 	teensy_loader_cli -w -mmcu=mk20dx256 $(HEX) -v
//! ```
//!
//! ### Complete example
//! You can find a complete example of the setup blinking a led here:
//! https://github.com/irevoire/teensy_blink

/// This module provide all the needed functions to boot the teensy
pub mod boot;
pub mod interrupts;
/// The Multipurpose Clock Generator
pub mod mcg;
/// The Oscillator Unit
pub mod osc;
/// The port, pins and gpio
pub mod port;
/// The System Integration Module
pub mod sim;
/// The watchdog is a piece of hardware which will reset the microcontroller unless the running
/// application “checks in” in a certain interval.
pub mod watchdog;

/// The only function you will need to implements
extern "Rust" {
    fn main();
}
