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

pub struct Gpio {
    gpio: *mut GpioBitband,
    pin: Pin,
}

impl Gpio {
    /// Create a Gpio. Before calling this function you should ensure that your pin is already in
    /// gpio mode. Prefer using function like `make_gpio` instead of calling this one directly.
    /// TODO: maybe we should move the `set_pin_mode` call from `make_gpio` to this function.
    /// This would allow direct call to this function
    pub unsafe fn new(pin: Pin) -> Gpio {
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
    pub fn input(&mut self) {
        unsafe {
            (*self.gpio).pddr[self.pin.id].write(0);
        }
    }

    /// Switch the pin in output mode (can write but not read)
    pub fn output(&mut self) {
        unsafe {
            (*self.gpio).pddr[self.pin.id].write(1);
        }
    }

    /// Before use, call the `input` function
    /// ```rust
    /// let pin = unsafe { port::Port::new(port::PortName::C).pin(3) };
    /// let mut gpio = pin.make_gpio();
    /// gpio.input();
    /// gpio.read();
    /// ```
    pub fn read(&mut self) -> u32 {
        unsafe { (*self.gpio).pdir[self.pin.id].read() }
    }

    /// Before use, call the `output` function
    /// ```rust
    /// let pin = unsafe { port::Port::new(port::PortName::C).pin(5) };
    /// let mut gpio = pin.make_gpio();
    /// gpio.output();
    /// gpio.high();
    /// ```
    pub fn high(&mut self) {
        unsafe {
            (*self.gpio).psor[self.pin.id].write(1);
        }
    }

    /// Before use, call the `output` function
    /// ```rust
    /// let pin = unsafe { port::Port::new(port::PortName::C).pin(5) };
    /// let mut gpio = pin.make_gpio();
    /// gpio.output();
    /// gpio.low();
    /// ```
    pub fn low(&mut self) {
        unsafe {
            (*self.gpio).pcor[self.pin.id].write(1);
        }
    }

    /// Before use, call the `output` function
    /// ```rust
    /// let pin = unsafe { port::Port::new(port::PortName::C).pin(5) };
    /// let mut gpio = pin.make_gpio();
    /// gpio.output();
    /// gpio.toggle();
    /// ```
    pub fn toggle(&mut self) {
        unsafe {
            (*self.gpio).ptor[self.pin.id].write(1);
        }
    }
}
