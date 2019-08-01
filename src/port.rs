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
    pub unsafe fn new(name: PortName) -> &'static mut Port {
        &mut *match name {
            PortName::A => 0x4004_9000 as *mut Port,
            PortName::B => 0x4004_A000 as *mut Port,
            PortName::C => 0x4004_B000 as *mut Port,
            PortName::D => 0x4004_C000 as *mut Port,
            PortName::E => 0x4004_D000 as *mut Port,
        }
    }

    pub unsafe fn set_pin_mode(&mut self, p: usize, mode: u32) {
        self.pcr[p].update(|pcr| {
            pcr.set_bits(8..=10, mode & 0b111); /* Update MUX field */
        });
    }

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
    pub fn make_gpio(self) -> Gpio {
        unsafe {
            let port = &mut *self.port;
            port.set_pin_mode(self.pin, 1);
            Gpio::new(port.name(), self.pin)
        }
    }
}

impl Gpio {
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

    /// switch the pin in input mode (can read but not write)
    pub fn input(&mut self) {
        unsafe {
            (*self.gpio).pddr[self.pin].write(0);
        }
    }

    /// switch the pin in output mode (can write but not read)
    pub fn output(&mut self) {
        unsafe {
            (*self.gpio).pddr[self.pin].write(1);
        }
    }

    /// before use call the `input` function
    pub fn read(&mut self) -> u32 {
        unsafe {
            (*self.gpio).pdir[self.pin].read()
        }
    }

    /// before use call the `output` function
    pub fn high(&mut self) {
        unsafe {
            (*self.gpio).psor[self.pin].write(1);
        }
    }

    /// before use call the `output` function
    pub fn low(&mut self) {
        unsafe {
            (*self.gpio).pcor[self.pin].write(1);
        }
    }

    /// before use call the `output` function
    pub fn toggle(&mut self) {
        unsafe {
            (*self.gpio).ptor[self.pin].write(1);
        }
    }
}
