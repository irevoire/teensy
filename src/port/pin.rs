use super::gpio::Gpio;
use super::port::{Port, PortName};

pub struct Pin {
    pub port: *mut Port,
    pub pin: usize,
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

    /// Put the pin in gpio mode and consume the pin into a gpio
    pub fn make_gpio(self) -> Gpio {
        unsafe {
            let port = &mut *self.port;
            port.set_pin_mode(self.pin, 1);
            Gpio::new(port.name(), self.pin)
        }
    }
}
