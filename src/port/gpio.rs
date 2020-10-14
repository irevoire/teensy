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

use core::marker::PhantomData;
pub struct Uninitialized {}
pub struct Input {}
pub struct Output {}

/// Chapter 49: General-Purpose Input/Output (GPIO)
/// doc/teensy_3.2.pdf - Page 1331
pub struct Gpio<State> {
    gpio: *mut GpioBitband,
    pin: Pin,
    _phantom: PhantomData<State>,
}

impl Gpio<Uninitialized> {
    /// Consume a Pin into a Gpio.
    pub unsafe fn new(mut pin: Pin) -> Gpio<Uninitialized> {
        pin.set_pin_mode(1); // put the pin in gpio mode
        let gpio = match pin.portname {
            PortName::A => 0x43FE_0000 as *mut GpioBitband,
            PortName::B => 0x43FE_0800 as *mut GpioBitband,
            PortName::C => 0x43FE_1000 as *mut GpioBitband,
            PortName::D => 0x43FE_1800 as *mut GpioBitband,
            PortName::E => 0x43FE_2000 as *mut GpioBitband,
        };

        Gpio {
            gpio,
            pin,
            _phantom: Default::default(),
        }
    }
}

impl<AnyState> Gpio<AnyState> {
    /// Switch the pin in input mode (can read but not write)
    /// See `input` function
    ///
    /// *This function can be implemented with a single write,
    /// eliminating the potential race condition. Thus its use is safe*
    pub fn input(self) -> Gpio<Input> {
        unsafe {
            (*self.gpio).pddr[self.pin.id].write(0);
        }
        Gpio {
            gpio: self.gpio,
            pin: self.pin,
            _phantom: Default::default(),
        }
    }

    /// Switch the pin in output mode (can write but not read)
    /// See `output` function
    ///
    /// *This function can be implemented with a single write,
    /// eliminating the potential race condition. Thus its use is safe*
    pub fn output(self) -> Gpio<Output> {
        unsafe {
            (*self.gpio).pddr[self.pin.id].write(1);
        }
        Gpio {
            gpio: self.gpio,
            pin: self.pin,
            _phantom: Default::default(),
        }
    }
}

use embedded_hal::digital;

impl digital::InputPin for Gpio<Input> {
    type Error = !;

    /// **Before use, call the `input` function**
    /// ```rust
    /// let pin = unsafe { port::Port::new(port::PortName::C).pin(3) };
    /// let mut gpio = pin.make_gpio();
    /// gpio.input();
    /// gpio.try_is_high();
    /// ```
    ///
    /// *This function can be implemented with a single write,
    /// eliminating the potential race condition. Thus its use is safe*
    fn try_is_high(&self) -> Result<bool, Self::Error> {
        let v = unsafe { (*self.gpio).pdir[self.pin.id].read() };
        Ok(v != 0)
    }

    /// **Before use, call the `input` function**
    /// ```rust
    /// let pin = unsafe { port::Port::new(port::PortName::C).pin(3) };
    /// let mut gpio = pin.make_gpio();
    /// gpio.input();
    /// gpio.try_is_low();
    /// ```
    ///
    /// *This function can be implemented with a single write,
    /// eliminating the potential race condition. Thus its use is safe*
    fn try_is_low(&self) -> Result<bool, Self::Error> {
        let v = unsafe { (*self.gpio).pdir[self.pin.id].read() };
        Ok(v == 0)
    }
}

impl digital::OutputPin for Gpio<Output> {
    type Error = !;

    /// **Before use, call the `output` function**
    /// ```rust
    /// let pin = unsafe { port::Port::new(port::PortName::C).pin(5) };
    /// let mut gpio = pin.make_gpio();
    /// gpio.output();
    /// gpio.try_set_high();
    /// ```
    ///
    /// *This function can be implemented with a single write,
    /// eliminating the potential race condition. Thus its use is safe*
    fn try_set_high(&mut self) -> Result<(), Self::Error> {
        unsafe {
            (*self.gpio).psor[self.pin.id].write(1);
        }
        Ok(())
    }

    /// **Before use, call the `output` function**
    /// ```rust
    /// let pin = unsafe { port::Port::new(port::PortName::C).pin(5) };
    /// let mut gpio = pin.make_gpio();
    /// gpio.output();
    /// gpio.try_set_low();
    /// ```
    ///
    /// *This function can be implemented with a single write,
    /// eliminating the potential race condition. Thus its use is safe*
    fn try_set_low(&mut self) -> Result<(), Self::Error> {
        unsafe {
            (*self.gpio).pcor[self.pin.id].write(1);
        }
        Ok(())
    }
}

impl digital::ToggleableOutputPin for Gpio<Output> {
    type Error = !;

    /// **Before use, call the `output` function**
    /// ```rust
    /// let pin = unsafe { port::Port::new(port::PortName::C).pin(5) };
    /// let mut gpio = pin.make_gpio();
    /// gpio.output();
    /// gpio.try_toggle();
    /// ```
    ///
    /// *This function can be implemented with a single write,
    /// eliminating the potential race condition. Thus its use is safe*
    fn try_toggle(&mut self) -> Result<(), Self::Error> {
        unsafe {
            (*self.gpio).ptor[self.pin.id].write(1);
        }
        Ok(())
    }
}
