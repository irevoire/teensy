use super::pin::Pin;
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
    /// a function handling this for you once you consumed your port into a pin (like `make_gpio`).
    pub unsafe fn set_pin_mode(&mut self, p: usize, mode: u32) {
        self.pcr[p].update(|pcr| {
            pcr.set_bits(8..=10, mode & 0b111); /* Update MUX field */
        });
    }

    /// enable pull resistor
    pub unsafe fn set_pin_pe(&mut self, p: usize, mode: bool) {
        assert!(p < 32);
        self.pcr[p].update(|pcr| {
            pcr.set_bit(1, mode);
        });
    }

    /// if pull resistor is enabled, pull up (1) or pull down (0)
    pub unsafe fn set_pin_ps(&mut self, p: usize, mode: bool) {
        assert!(p < 32);
        self.pcr[p].update(|pcr| {
            pcr.set_bit(0, mode);
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
