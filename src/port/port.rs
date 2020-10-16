use super::pin::Pin;
use super::{A, B, C, D, E};
use core::marker::PhantomData;
use volatile::Volatile;

/// 11.1.4 Memory map and register definition
/// doc/teensy_3.2.pdf - Page 221
#[repr(C, packed)]
pub struct Port<Name> {
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

    _phantom: PhantomData<Name>,
}

impl Port<A> {
    /// Create a new port. Take a port name as argument.
    pub unsafe fn new() -> &'static mut Self {
        &mut *(0x4004_9000 as *mut Self)
    }
}

impl Port<B> {
    /// Create a new port. Take a port name as argument.
    pub unsafe fn new() -> &'static mut Self {
        &mut *(0x4004_A000 as *mut Self)
    }
}
impl Port<C> {
    /// Create a new port. Take a port name as argument.
    pub unsafe fn new() -> &'static mut Self {
        &mut *(0x4004_B000 as *mut Self)
    }
}

impl Port<D> {
    /// Create a new port. Take a port name as argument.
    pub unsafe fn new() -> &'static mut Self {
        &mut *(0x4004_C000 as *mut Self)
    }
}

impl Port<E> {
    /// Create a new port. Take a port name as argument.
    pub unsafe fn new() -> &'static mut Self {
        &mut *(0x4004_D000 as *mut Self)
    }
}

impl<Name> Port<Name> {
    /// Consume the port into a pin
    /// The pin keep a reference to it's pcr field
    pub unsafe fn pin(&'static mut self, p: usize) -> Pin<Name> {
        assert!(p < 32);
        Pin {
            pcr: &mut self.pcr[p],
            id: p,
            _phantom: Default::default(),
        }
    }
}
