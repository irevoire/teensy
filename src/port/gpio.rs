use super::pin::Pin;
use super::{A, B, C, D, E};
use core::marker::PhantomData;
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

pub struct Uninitialized {}
pub struct Input {}
pub struct Output {}

/// Chapter 49: General-Purpose Input/Output (GPIO)
/// doc/teensy_3.2.pdf - Page 1331
pub struct Gpio<State, PortName> {
    gpio: *mut GpioBitband,
    pin: Pin<PortName>,
    _phantom: PhantomData<State>,
    _phantom2: PhantomData<PortName>,
}

impl<PortName> Gpio<Uninitialized, PortName> {
    /// Consume a Pin into a Gpio.
    pub unsafe fn new(mut pin: Pin<PortName>) -> Self {
        pin.set_pin_mode(1); // put the pin in gpio mode
        unreachable!();
        /*
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
        */
    }
}

impl<AnyState, PortName> Gpio<AnyState, PortName> {
    /// Switch the pin in input mode (can read but not write)
    /// See `input` function
    ///
    /// *This function can be implemented with a single write,
    /// eliminating the potential race condition. Thus its use is safe*
    pub fn input(self) -> Gpio<Input, PortName> {
        unsafe {
            (*self.gpio).pddr[self.pin.id].write(0);
        }
        Gpio {
            gpio: self.gpio,
            pin: self.pin,
            _phantom: Default::default(),
            _phantom2: Default::default(),
        }
    }

    /// Switch the pin in output mode (can write but not read)
    /// See `output` function
    ///
    /// *This function can be implemented with a single write,
    /// eliminating the potential race condition. Thus its use is safe*
    pub fn output(self) -> Gpio<Output, PortName> {
        unsafe {
            (*self.gpio).pddr[self.pin.id].write(1);
        }
        Gpio {
            gpio: self.gpio,
            pin: self.pin,
            _phantom: Default::default(),
            _phantom2: Default::default(),
        }
    }
}

use embedded_hal::digital;

impl<PortName> digital::InputPin for Gpio<Input, PortName> {
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

impl<PortName> digital::OutputPin for Gpio<Output, PortName> {
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

impl<PortName> digital::ToggleableOutputPin for Gpio<Output, PortName> {
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
