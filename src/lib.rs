#![allow(dead_code)]
#![feature(stdsimd)]
#![no_std]

//! # Teensy
//! This crate provide multiple helpers functions to manipulate the teensy in baremetal rust.
//! Currently everything is tested on a teensy 3.2. It provide:
//! * A bootloader
//! * Helper to handle the gpio pins
//!
//! # Quick start
//! ## Get the dependencies
//! - The targetted architecture
//! ```sh
//! # archlinux
//! sudo pacman -S arm-none-eabi-binutils
//! # ubuntu
//! sudo apt install binutils-arm-none-eabi
//! # macOS
//! brew install gcc-arm-none-eabi
//! ```
//! - The loader to flash the teensy
//! ```sh
//! # archlinux aur
//! yay -S teensy_loader_cli
//! # or
//! yaourt -S teensy_loader_cli
//! # macOS
//! brew install teensy_loader_cli
//! ```
//! For other distribution or if you want to avoid the aur package you can manually install it from
//! [here](https://github.com/PaulStoffregen/teensy_loader_cli)
//!
//! ## Prepare your environment
//! 1. Add the crate as a dependency
//! Currently the crate is still under heavy development so there is no release on crate.io.
//! To include this crate in your project add this line to the dependency section of your
//! `Cargo.toml` file:
//! ```toml
//! teensy = { git = "https://github.com/irevoire/teensy.git"}
//! ```
//! 2. Nightly rust
//! Finally to compile with this crate you need to use the nightly channel, just run:
//! ```bash
//! rustup override set nightly
//! ```
//! 3. The correct target
//! You’ll not compile your code for the casual x86 assembly. You'll need to install a new target
//! for arm processor:
//! ```bash
//! rustup target add thumbv7em-none-eabi
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
//! EXTERN(_INTERRUPTS);
//! SECTIONS
//! {
//!     PROVIDE(_stack_top = ORIGIN(RAM) + LENGTH(RAM));
//!     .vector_table ORIGIN(FLASH) : {
//!         LONG(_stack_top);
//!         KEEP(*(.vector_table.interrupts));
//!     } > FLASH
//! 	.text : {
//! 		. = 0x400;
//! 		KEEP(*(.flashconfig*))
//! 		. = ALIGN(4);
//! 		*(.text*)
//!     } > FLASH = 0xFF
//!     .rodata : ALIGN(4){
//! 	    *(.rodata .rodata.*);
//! 	    . = ALIGN(4);
//!     } > FLASH
//!     /DISCARD/ : {
//! 	    *(.ARM.*)
//!     }
//! }
//! ```
//!
//! **This script is also available with comments in the repository as
//! [`layout.ld`](https://github.com/irevoire/teensy/blob/master/layout.ld).**
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
//! ## Code with this crate
//! Here is a minimal example of valid code:
//! ```rust
//! #![no_std]
//! #![no_main]
//!
//! // needed because we have no std to handle the panic
//! teensy::define_panic!{empty}
//!
//! #[no_mangle]
//! fn main() {
//!     loop {}
//! }
//! ```
//!
//! ## Flashing the Teensy
//! We provide a Makefile to generate a binary for the teensy 3.2 and sending it using the
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
//! You can find a complete example of the setup blinking a led
//! [here](https://github.com/irevoire/teensy_blink)
//!
//! # Advanced usage
//! ## Choosing yourself which components are used
//! If you use the crate without specifying anything, as seen in the quickstart guide,
//! every components will be enbled and the teensy will run at max clock speed.
//! This crate provide a feature `manual_init` to provide your own initialization function.
//! You can see the `examples/blink_manual_init.rs` file to see how to use the feature in your
//! code.
//! You’ll also need to import the crate with the feature enabled:
//! ```toml
//! [dependencies.teensy]
//! git = "https://github.com/irevoire/teensy.git"
//! default-features = false
//! features = ["manual_init"]
//! ```
//! [Here is an example of repository showing the usage of this feature.
//! ](https://github.com/irevoire/teensy_blink_manual)
//!

/// This module provide all the needed functions to boot the teensy.
pub mod boot;
pub mod interrupts;
/// The Multipurpose Clock Generator.
pub mod mcg;
/// The Oscillator Unit.
pub mod osc;
/// Helper module to define easily panic function.
pub mod panic;
/// The port, pins and gpio.
pub mod port;
/// The System Integration Module.
pub mod sim;
/// The watchdog is a piece of hardware which will reset the microcontroller unless the running
/// application “checks in” in a certain interval.
pub mod watchdog;

// I think the next modules should be moved to a kindof "common" module
/// Implementation of the sleep function
pub mod sleep;

/// The only function you will need to implements
extern "Rust" {
    fn init();
    fn main();
}
