//! There are two ways to access the GPIO registers. The first is through a block of 32-bit
//! registers, associated with a port. It looks something like this:
//! ```rust
//! #[repr(C,packed)]
//! struct Gpio {
//!     pdor: u32,
//!     psor: u32,
//!     pcor: u32,
//!     ptor: u32,
//!     pdir: u32,
//!     pddr: u32
//! }
//! ```
//!
//! This is very convenient to work with, but has an unfortunate flaw. Each of the fields
//! represents all 32 pins in a Port. This means that any pin changes are subject to a race
//! condition during our read/modify/write process. Pins that are owned by a separate piece of
//! code can have an impact on how our pin behaves.
//! Fortunately, ARM has a solution to this. We will take advantage of the bit-band alias.
//! Bit-banding is a feature of certain ARM processors that maps a memory region to one 32 times
//! as large. Each 32-bit word of this larger regions maps to a single bit of the original region.
//! This gives us the capability to set or clear a single bit at a time, without risk of race
//! conditions. If we visualized this as a rust struct, the bit-band alias for the GPIO would look
//! like this:
//! ```rust
//! #[repr(C,packed)]
//! struct GpioBitband {
//!     pdor: [u32; 32],
//!     psor: [u32; 32],
//!     pcor: [u32; 32],
//!     ptor: [u32; 32],
//!     pdir: [u32; 32],
//!     pddr: [u32; 32]
//! }
//! ```
//!
//! If you are here it's probably because you want to use some pins, here is the step you need to
//! follow:
//! 1. Identifiying in which port the pin you want to use is located, try looking at this
//!    documents: [teensy schematic representation](https://github.com/irevoire/teensy/blob/master/doc/schematic.gif)
//!    For example imagine we want to lighten up the led. On this document we can see that the led
//!    is connected to the pin number 13 (on the top right). And at the base of this pin we can
//!    read "PT C5", which means that the pin belongs to the port C and is the number 5.
//!    We now know we want to use the: `port::PortName::C`.
//! 2. Creating a port: `let port = port::Port::new(port::PortName::C)`
//! 3. Consumming the port into the right pin: `let pin = port.pin(5)`
//! 4. Choosing a mode for the pin, here we want to do normal gpio: `let mut led = pin.make_gpio()`
//! 5. Configuring the pin; we want to write data in the pin: `gpio.output()`
//! 6. Light the pin: `gpio.high()`
//! 7. Enjoy 🎉
//! ```rust
//! let pin = unsafe { port::Port::new(port::PortName::C).pin(5) };
//! let mut gpio = pin.make_gpio();
//! gpio.output();
//! gpio.high();
//! ```

pub use self::pin::Pin;
pub use self::port::Port;
pub use self::port::PortName;
pub use self::gpio::Gpio;

mod gpio;
mod pin;
mod port;

pub mod helpers;
