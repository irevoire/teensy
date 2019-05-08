use core;

#[derive(Clone, Copy)]
pub enum PortName {
    A,
    B,
    C,
    D,
    E,
}

#[repr(C, packed)]
pub struct Port {
    pcr: [u32; 32],
    gpclr: u32,
    gpchr: u32,
    reserved_0: [u8; 24],
    isfr: u32,
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

    pub unsafe fn set_pin_mode(&mut self, p: usize, mut mode: u32) {
        let mut pcr = core::ptr::read_volatile(&self.pcr[p]);
        pcr &= !0x0000_0700;
        mode &= 0x0000_0007;
        mode <<= 8;
        pcr |= mode;
        core::ptr::write_volatile(&mut self.pcr[p], pcr);
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
    pdor: [u32; 32],
    psor: [u32; 32],
    pcor: [u32; 32],
    ptor: [u32; 32],
    pdir: [u32; 32],
    pddr: [u32; 32],
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
            PortName::A => 0x43FE_0600 as *mut GpioBitband,
            PortName::B => 0x43FE_0800 as *mut GpioBitband,
            PortName::C => 0x43FE_1000 as *mut GpioBitband,
            PortName::D => 0x43FE_1200 as *mut GpioBitband,
            PortName::E => 0x43FE_1400 as *mut GpioBitband,    
        };

        Gpio { gpio, pin }
    }

    pub fn input(&mut self) {
        unsafe {
            core::ptr::write_volatile(&mut (*self.gpio).pddr[self.pin], 0);
        }
    }

    pub fn output(&mut self) {
        unsafe {
            core::ptr::write_volatile(&mut (*self.gpio).pddr[self.pin], 1);
        }
    }

    /// before use call the `input` function
    pub fn read(&mut self) -> u32 {
        unsafe { core::ptr::read_volatile(&mut (*self.gpio).pdir[self.pin]) }
    }

    /// before use call the `output` function
    pub fn high(&mut self) {
        unsafe {
            core::ptr::write_volatile(&mut (*self.gpio).psor[self.pin], 1);
        }
    }

    /// before use call the `output` function
    pub fn low(&mut self) {
        unsafe {
            core::ptr::write_volatile(&mut (*self.gpio).pcor[self.pin], 1);
        }
    }

    /// before use call the `output` function
    pub fn toggle(&mut self) {
        unsafe {
            core::ptr::write_volatile(&mut (*self.gpio).ptor[self.pin], 1);
        }
    }
}
