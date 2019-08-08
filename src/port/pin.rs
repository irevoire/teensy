use super::gpio::Gpio;
use super::port::{Port, PortName};
use bit_field::BitField;
use volatile::Volatile;

pub struct Pin {
    pub portname: PortName,
    pub id: usize,
    pub pcr: &'static mut Volatile<u32>, // pcr should point to a part of port
}

impl Pin {
    /// Create a pin from the pin number indicated in the
    /// [schematic representation of the teensy](https://github.com/irevoire/teensy/blob/master/doc/schematic.gif)
    /// Will panic if the number is more than 33
    pub unsafe fn new(number: usize) -> Self {
        match number {
            0 => Port::new(PortName::B).pin(16),
            1 => Port::new(PortName::B).pin(17),
            2 => Port::new(PortName::D).pin(0),
            3 => Port::new(PortName::A).pin(12),
            4 => Port::new(PortName::A).pin(13),
            5 => Port::new(PortName::D).pin(7),
            6 => Port::new(PortName::D).pin(4),
            7 => Port::new(PortName::D).pin(2),
            8 => Port::new(PortName::D).pin(3),
            9 => Port::new(PortName::C).pin(3),
            10 => Port::new(PortName::C).pin(4),
            11 => Port::new(PortName::C).pin(6),
            12 => Port::new(PortName::C).pin(7),

            13 => Port::new(PortName::C).pin(5),
            14 => Port::new(PortName::D).pin(1),
            15 => Port::new(PortName::C).pin(0),
            16 => Port::new(PortName::B).pin(0),
            17 => Port::new(PortName::B).pin(1),
            18 => Port::new(PortName::B).pin(3),
            19 => Port::new(PortName::B).pin(2),
            20 => Port::new(PortName::D).pin(5),
            21 => Port::new(PortName::D).pin(6),
            22 => Port::new(PortName::C).pin(1),
            23 => Port::new(PortName::C).pin(2),

            24 => Port::new(PortName::A).pin(5),
            25 => Port::new(PortName::B).pin(19),
            26 => Port::new(PortName::E).pin(1),
            27 => Port::new(PortName::C).pin(9),
            28 => Port::new(PortName::C).pin(8),
            29 => Port::new(PortName::C).pin(10),
            30 => Port::new(PortName::C).pin(11),
            31 => Port::new(PortName::E).pin(0),
            32 => Port::new(PortName::B).pin(18),
            33 => Port::new(PortName::A).pin(4),
            _ => panic!("Bad pin number!"),
        }
    }

    /// update the mode of the pin. You should not use this function directly and look if there is
    /// a function handling this for you once you consumed your port into a pin (like `make_gpio`).
    pub unsafe fn set_pin_mode(&mut self, mode: u32) {
        self.pcr.update(|pcr| {
            pcr.set_bits(8..=10, mode & 0b111); /* Update MUX field */
        });
    }

    /// enable pull resistor
    pub unsafe fn set_pin_pe(&mut self, mode: bool) {
        self.pcr.update(|pcr| {
            pcr.set_bit(1, mode);
        });
    }

    /// if pull resistor is enabled, pull up (1) or pull down (0)
    pub unsafe fn set_pin_ps(&mut self, mode: bool) {
        self.pcr.update(|pcr| {
            pcr.set_bit(0, mode);
        });
    }

    /// Put the pin in gpio mode and consume the pin into a gpio
    pub fn make_gpio(self) -> Gpio {
        unsafe { Gpio::new(self) }
    }
}
