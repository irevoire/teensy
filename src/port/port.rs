use super::pin::Pin;
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
    pub pcr: [Volatile<u32>; 32],
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
    /// The pin keep a reference to it's pcr field
    pub unsafe fn pin(&'static mut self, p: usize) -> Pin {
        assert!(p < 32);
        Pin {
            portname: self.name(),
            pcr: &mut self.pcr[p],
            id: p,
        }
    }
}
