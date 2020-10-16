use super::gpio::{Gpio, Uninitialized};
use super::port::Port;
use super::{A, B, C, D, E};
use bit_field::BitField;
use core::marker::PhantomData;
use volatile::Volatile;

pub struct Pin<PortName> {
    pub id: usize,
    pub pcr: &'static mut Volatile<u32>, // pcr should point to a part of port

    pub _phantom: PhantomData<PortName>,
}

impl Pin<A> {
    /// Create a pin from the pin number indicated in the
    /// [schematic representation of the teensy](https://github.com/irevoire/teensy/blob/master/doc/schematic.gif)
    /// Will panic if the number is more than 33
    pub unsafe fn new(number: usize) -> Self {
        match number {
            3 => Port::<A>::new().pin(12),
            4 => Port::<A>::new().pin(13),
            24 => Port::<A>::new().pin(5),
            33 => Port::<A>::new().pin(4),
            _ => panic!("Bad pin number!"),
        }
    }
}

impl Pin<B> {
    /// Create a pin from the pin number indicated in the
    /// [schematic representation of the teensy](https://github.com/irevoire/teensy/blob/master/doc/schematic.gif)
    /// Will panic if the number is more than 33
    pub unsafe fn new(number: usize) -> Self {
        match number {
            0 => Port::<B>::new().pin(16),
            1 => Port::<B>::new().pin(17),
            16 => Port::<B>::new().pin(0),
            17 => Port::<B>::new().pin(1),
            18 => Port::<B>::new().pin(3),
            19 => Port::<B>::new().pin(2),
            25 => Port::<B>::new().pin(19),
            32 => Port::<B>::new().pin(18),
            _ => panic!("Bad pin number!"),
        }
    }
}

impl Pin<C> {
    /// Create a pin from the pin number indicated in the
    /// [schematic representation of the teensy](https://github.com/irevoire/teensy/blob/master/doc/schematic.gif)
    /// Will panic if the number is more than 33
    pub unsafe fn new(number: usize) -> Self {
        match number {
            9 => Port::<C>::new().pin(3),
            10 => Port::<C>::new().pin(4),
            11 => Port::<C>::new().pin(6),
            12 => Port::<C>::new().pin(7),

            13 => Port::<C>::new().pin(5),
            15 => Port::<C>::new().pin(0),
            22 => Port::<C>::new().pin(1),
            23 => Port::<C>::new().pin(2),

            27 => Port::<C>::new().pin(9),
            28 => Port::<C>::new().pin(8),
            29 => Port::<C>::new().pin(10),
            30 => Port::<C>::new().pin(11),
            _ => panic!("Bad pin number!"),
        }
    }
}

impl Pin<D> {
    /// Create a pin from the pin number indicated in the
    /// [schematic representation of the teensy](https://github.com/irevoire/teensy/blob/master/doc/schematic.gif)
    /// Will panic if the number is more than 33
    pub unsafe fn new(number: usize) -> Self {
        match number {
            2 => Port::<D>::new().pin(0),
            5 => Port::<D>::new().pin(7),
            6 => Port::<D>::new().pin(4),
            7 => Port::<D>::new().pin(2),
            8 => Port::<D>::new().pin(3),
            14 => Port::<D>::new().pin(1),
            20 => Port::<D>::new().pin(5),
            21 => Port::<D>::new().pin(6),
            _ => panic!("Bad pin number!"),
        }
    }
}

impl Pin<E> {
    /// Create a pin from the pin number indicated in the
    /// [schematic representation of the teensy](https://github.com/irevoire/teensy/blob/master/doc/schematic.gif)
    /// Will panic if the number is more than 33
    pub unsafe fn new(number: usize) -> Self {
        match number {
            26 => Port::<E>::new().pin(1),
            31 => Port::<E>::new().pin(0),
            _ => panic!("Bad pin number!"),
        }
    }
}

impl<PortName> Pin<PortName> {
    /// update the mode of the pin. You should not use this function directly and look if there is
    /// a function handling this for you once you consumed your port into a pin (like `make_gpio`).
    pub unsafe fn set_pin_mode(&mut self, mode: u32) {
        self.pcr.update(|pcr| {
            pcr.set_bits(8..=10, mode & 0b111); /* Update MUX field */
        });
    }

    /// update the mode of the pin. You should not use this function directly and look if there is
    /// a function handling this for you once you consumed your port into a pin (like `make_gpio`).
    pub unsafe fn with_pin_mode(mut self, mode: u32) -> Self {
        self.set_pin_mode(mode);
        self
    }

    /// Pull Select
    ///
    /// This bit is read only for pins that do not support a configurable pull
    /// resistor direction. Pull configuration is valid in all digital pin
    /// muxing modes.
    ///
    /// - `false`: Internal pulldown resistor is enabled on the corresponding pin,
    /// if the corresponding Port Pull Enable field is set.
    /// - `true`:  Internal pullup resistor is enabled on the corresponding pin,
    /// if the corresponding Port Pull Enable field is set.
    pub unsafe fn set_pin_ps(&mut self, mode: bool) {
        self.pcr.update(|pcr| {
            pcr.set_bit(0, mode);
        });
    }

    /// Refer to the `set_pin_ps` method
    pub unsafe fn with_pin_ps(mut self, mode: bool) -> Self {
        self.set_pin_ps(mode);
        self
    }

    /// Pull Enable
    ///
    /// This bit is read only for pins that do not support a configurable pull
    /// resistor. Pull configuration is valid in all digital pin muxing modes.
    ///
    /// - `false`: Internal pullup or pulldown resistor is not enabled on the
    /// corresponding pin.
    /// - `true`: Internal pullup or pulldown resistor is enabled on the
    /// corresponding pin, if the pin is configured as a digital input.
    pub unsafe fn set_pin_pe(&mut self, mode: bool) {
        self.pcr.update(|pcr| {
            pcr.set_bit(1, mode);
        });
    }

    /// Refer to the `set_pin_pe` method
    pub unsafe fn with_pin_pe(mut self, mode: bool) -> Self {
        self.set_pin_pe(mode);
        self
    }

    /// Slew Rate Enable
    ///
    /// This bit is read only for pins that do not support a configurable slew
    /// rate. Slew rate configuration is valid in all digital pin muxing modes.
    ///
    /// - `false`: Fast slew rate is configured on the corresponding pin, if
    /// the pin is configured as a digital output.
    /// - `true`: Slow slew rate is configured on the corresponding pin, if
    /// the pin is configured as a digital output.
    pub unsafe fn set_pin_sre(&mut self, mode: bool) {
        self.pcr.update(|pcr| {
            pcr.set_bit(2, mode);
        });
    }

    /// Refer to the `set_pin_sre` method
    pub unsafe fn with_pin_sre(mut self, mode: bool) -> Self {
        self.set_pin_sre(mode);
        self
    }

    /// Passive Filter Enable
    ///
    /// This bit is read only for pins that do not support a configurable passive
    /// input filter. Passive filter configuration is valid in all digital pin muxing
    /// modes.
    ///
    /// - `false`: Passive input filter is disabled on the corresponding pin.
    /// - `true`:  Passive input filter is enabled on the corresponding pin, if the
    /// pin is configured as a digital input. A low pass filter of 10 MHz to 30 MHz
    /// bandwidth is enabled on the digital input path. Disable the passive input
    /// filter when high speed interfaces of more than 2 MHz are supported on the pin.
    pub unsafe fn set_pin_pfe(&mut self, mode: bool) {
        self.pcr.update(|pcr| {
            pcr.set_bit(4, mode);
        });
    }

    /// Refer to the `set_pin_pfe` method
    pub unsafe fn with_pin_pfe(mut self, mode: bool) -> Self {
        self.set_pin_pfe(mode);
        self
    }

    /// Open Drain Enable
    ///
    /// This bit is read only for pins that do not support a configurable open
    /// drain output. Open drain configuration is valid in all digital pin
    /// muxing modes.
    ///
    /// - `false`: Open drain output is disabled on the corresponding pin.
    /// - `true`: Open drain output is enabled on the corresponding pin, if the
    /// pin is configured as a digital output.
    pub unsafe fn set_pin_ode(&mut self, mode: bool) {
        self.pcr.update(|pcr| {
            pcr.set_bit(5, mode);
        });
    }

    /// Refer to the `set_pin_ode` method
    pub unsafe fn with_pin_ode(mut self, mode: bool) -> Self {
        self.set_pin_ode(mode);
        self
    }

    /// Drive Strength Enable
    ///
    /// This bit is read only for pins that do not support a configurable drive
    /// strength. Drive strength configuration is valid in all digital pin
    /// muxing modes.
    ///
    /// - `false`:  Low drive strength is configured on the corresponding pin,
    /// if pin is configured as a digital output.
    /// - `true`:   High drive strength is configured on the corresponding pin,
    /// if pin is configured as a digital output.
    pub unsafe fn set_pin_dse(&mut self, mode: bool) {
        self.pcr.update(|pcr| {
            pcr.set_bit(6, mode);
        });
    }

    /// Refer to the `set_pin_dse` method
    pub unsafe fn with_pin_dse(mut self, mode: bool) -> Self {
        self.set_pin_pfe(mode);
        self
    }

    /// Put the pin in gpio mode and consume the pin into a gpio
    pub fn make_gpio(self) -> Gpio<Uninitialized, PortName> {
        unsafe { Gpio::new(self) }
    }
}
