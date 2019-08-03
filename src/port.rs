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

use bit_field::BitField;
use volatile::Volatile;

#[derive(Clone, Copy)]
pub enum PortName {
    A,
    B,
    C,
    D,
    E,
}

/// 11.1.4 Memory map and register definition
/// doc/teensy_3.2.pdf - Page 221
#[repr(C, packed)]
pub struct Port {
    /// 11.14.1 Pin Control Register n (PORTx_PCRn)
    /// doc/teensy_3.2.pdf - Page 227
    /* One for each pin on this port
       Bits 8-10 : MUX
       */
    pcr: [Volatile<u32>; 32],
    gpclr: Volatile<u32>,
    gpchr: Volatile<u32>,
    _reserved: Volatile<[u8; 24]>,
    isfr: Volatile<u32>,
}

impl Port {
    /// Create a new port. Take a port name as argument.
    pub unsafe fn new(name: PortName) -> &'static mut Port {
        &mut *match name {
            PortName::A => 0x4004_9000 as *mut Port,
            PortName::B => 0x4004_A000 as *mut Port,
            PortName::C => 0x4004_B000 as *mut Port,
            PortName::D => 0x4004_C000 as *mut Port,
            PortName::E => 0x4004_D000 as *mut Port,
        }
    }

    /// update the mode of the pin. You should not use this function directly and look if there is
    /// a function handling this for you once you consummed your port into a pin (like `make_gpio`).
    pub unsafe fn set_pin_mode(&mut self, p: usize, mode: u32) {
        self.pcr[p].update(|pcr| {
            pcr.set_bits(8..=10, mode & 0b111); /* Update MUX field */
        });
    }

    /// Retrieve the portname associated to the port
    pub fn name(&self) -> PortName {
        let addr = (self as *const Port) as u32;
        match addr {
            0x4004_9000 => PortName::A,
            0x4004_A000 => PortName::B,
            0x4004_B000 => PortName::C,
            0x4004_C000 => PortName::D,
            0x4004_D000 => PortName::E,
            _ => unreachable!(),
        }
    }

    /// Consume the port into a pin
    pub unsafe fn pin(&mut self, p: usize) -> Pin {
        Pin { port: self, pin: p }
    }
}

#[repr(C, packed)]
struct GpioBitband {
    pdor: [Volatile<u32>; 32],
    psor: [Volatile<u32>; 32],
    pcor: [Volatile<u32>; 32],
    ptor: [Volatile<u32>; 32],
    pdir: [Volatile<u32>; 32],
    pddr: [Volatile<u32>; 32],
}

pub struct Gpio {
    gpio: *mut GpioBitband,
    pin: usize,
}

pub struct Pin {
    port: *mut Port,
    pin: usize,
}

impl Pin {
    /// Put the pin in gpio mode and consume the pin into a gpio
    pub fn make_gpio(self) -> Gpio {
        unsafe {
            let port = &mut *self.port;
            port.set_pin_mode(self.pin, 1);
            Gpio::new(port.name(), self.pin)
        }
    }
}

impl Gpio {
    /// Create a Gpio. Before calling this function you should ensure that your pin is already in
    /// gpio mode. Prefer using function like `make_gpio` instead of calling this one directly.
    /// TODO: maybe we should move the `set_pin_mode` call from `make_gpio` to this function.
    /// This would allow direct call to this function
    pub unsafe fn new(port: PortName, pin: usize) -> Gpio {
        let gpio = match port {
            PortName::A => 0x43FE_0000 as *mut GpioBitband,
            PortName::B => 0x43FE_0800 as *mut GpioBitband,
            PortName::C => 0x43FE_1000 as *mut GpioBitband,
            PortName::D => 0x43FE_1800 as *mut GpioBitband,
            PortName::E => 0x43FE_2000 as *mut GpioBitband,
        };

        Gpio { gpio, pin }
    }

    /// Switch the pin in input mode (can read but not write)
    pub fn input(&mut self) {
        unsafe {
            (*self.gpio).pddr[self.pin].write(0);
        }
    }

    /// Switch the pin in output mode (can write but not read)
    pub fn output(&mut self) {
        unsafe {
            (*self.gpio).pddr[self.pin].write(1);
        }
    }

    /// Before use call the `input` function
    /// ```rust
    /// let pin = unsafe { port::Port::new(port::PortName::C).pin(5) };
    /// let mut gpio = pin.make_gpio();
    /// gpio.input();
    /// gpio.read();
    /// ```
    pub fn read(&mut self) -> u32 {
        unsafe { (*self.gpio).pdir[self.pin].read() }
    }

    /// Before use call the `output` function
    /// ```rust
    /// let pin = unsafe { port::Port::new(port::PortName::C).pin(5) };
    /// let mut gpio = pin.make_gpio();
    /// gpio.output();
    /// gpio.high();
    /// ```
    pub fn high(&mut self) {
        unsafe {
            (*self.gpio).psor[self.pin].write(1);
        }
    }

    /// Before use call the `output` function
    /// ```rust
    /// let pin = unsafe { port::Port::new(port::PortName::C).pin(5) };
    /// let mut gpio = pin.make_gpio();
    /// gpio.output();
    /// gpio.low();
    /// ```
    pub fn low(&mut self) {
        unsafe {
            (*self.gpio).pcor[self.pin].write(1);
        }
    }

    /// Before use call the `output` function
    /// ```rust
    /// let pin = unsafe { port::Port::new(port::PortName::C).pin(5) };
    /// let mut gpio = pin.make_gpio();
    /// gpio.output();
    /// gpio.toggle();
    /// ```
    pub fn toggle(&mut self) {
        unsafe {
            (*self.gpio).ptor[self.pin].write(1);
        }
    }
}
