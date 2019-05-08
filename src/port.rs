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

/// doc/teensy_3.2.pdf - Page 221
/// 11.1.4 Memory map and register definition
#[repr(C, packed)]
pub struct Port {
    /// doc/teensy_3.2.pdf - Page 227
    /// 11.14.1 Pin Control Register n (PORTx_PCRn)

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

    pub fn input(&mut self) {
        unsafe {
            (*self.gpio).pddr[self.pin].write(0);
        }
    }

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
