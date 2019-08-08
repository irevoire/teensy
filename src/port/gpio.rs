use super::pin::Pin;
use super::port::PortName;
use volatile::Volatile;

#[repr(C, packed)]
struct GpioBitband {
    pdor: [Volatile<u32>; 32],
    psor: [Volatile<u32>; 32],
    pcor: [Volatile<u32>; 32],
    ptor: [Volatile<u32>; 32],
    pdir: [Volatile<u32>; 32],
    pddr: [Volatile<u32>; 32],
}

/// Chapter 49: General-Purpose Input/Output (GPIO)
/// doc/teensy_3.2.pdf - Page 1331
pub struct Gpio {
    gpio: *mut GpioBitband,
    pin: Pin,
}

impl Gpio {
    /// Consume a Pin into a Gpio.
    pub unsafe fn new(mut pin: Pin) -> Gpio {
        pin.set_pin_mode(1); // put the pin in gpio mode
        let gpio = match pin.portname {
            PortName::A => 0x43FE_0000 as *mut GpioBitband,
            PortName::B => 0x43FE_0800 as *mut GpioBitband,
            PortName::C => 0x43FE_1000 as *mut GpioBitband,
            PortName::D => 0x43FE_1800 as *mut GpioBitband,
            PortName::E => 0x43FE_2000 as *mut GpioBitband,
        };

        Gpio { gpio, pin }
    }

    /// Switch the pin in input mode (can read but not write)
    ///
    /// *This function can be implemented with a single write,
    /// eliminating the potential race condition. Thus its use is safe*
    pub fn input(&mut self) {
        unsafe {
            (*self.gpio).pddr[self.pin.id].write(0);
        }
    }

    /// Switch the pin in output mode (can write but not read)
    ///
    /// *This function can be implemented with a single write,
    /// eliminating the potential race condition. Thus its use is safe*
    pub fn output(&mut self) {
        unsafe {
            (*self.gpio).pddr[self.pin.id].write(1);
        }
    }

    /// **Before use, call the `input` function**
    /// ```rust
    /// let pin = unsafe { port::Port::new(port::PortName::C).pin(3) };
    /// let mut gpio = pin.make_gpio();
    /// gpio.input();
    /// gpio.read();
    /// ```
    ///
    /// *This function can be implemented with a single write,
    /// eliminating the potential race condition. Thus its use is safe*
    pub fn read(&mut self) -> u32 {
        unsafe { (*self.gpio).pdir[self.pin.id].read() }
    }

    /// **Before use, call the `output` function**
    /// ```rust
    /// let pin = unsafe { port::Port::new(port::PortName::C).pin(5) };
    /// let mut gpio = pin.make_gpio();
    /// gpio.output();
    /// gpio.high();
    /// ```
    ///
    /// *This function can be implemented with a single write,
    /// eliminating the potential race condition. Thus its use is safe*
    pub fn high(&mut self) {
        unsafe {
            (*self.gpio).psor[self.pin.id].write(1);
        }
    }

    /// **Before use, call the `output` function**
    /// ```rust
    /// let pin = unsafe { port::Port::new(port::PortName::C).pin(5) };
    /// let mut gpio = pin.make_gpio();
    /// gpio.output();
    /// gpio.low();
    /// ```
    ///
    /// *This function can be implemented with a single write,
    /// eliminating the potential race condition. Thus its use is safe*
    pub fn low(&mut self) {
        unsafe {
            (*self.gpio).pcor[self.pin.id].write(1);
        }
    }

    /// **Before use, call the `output` function**
    /// ```rust
    /// let pin = unsafe { port::Port::new(port::PortName::C).pin(5) };
    /// let mut gpio = pin.make_gpio();
    /// gpio.output();
    /// gpio.toggle();
    /// ```
    ///
    /// *This function can be implemented with a single write,
    /// eliminating the potential race condition. Thus its use is safe*
    pub fn toggle(&mut self) {
        unsafe {
            (*self.gpio).ptor[self.pin.id].write(1);
        }
    }
}
