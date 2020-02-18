/// This macro is an helper to create pin easily. You can create your pins by the name they have on
/// the little flyer you got when you bought your teensy. You can create your pin with name like:
/// ```rust
/// let pin = make_pin!(led);
/// let pin = make_pin!(23);
/// let pin = make_pin!(A8);
/// let pin = make_pin!(TX2);
/// let pin = make_pin!(DOUT);
/// let pin = make_pin!(SCL0);
/// let pin = make_pin!(PTD3); // from the schematic view
/// ```
/// You can also call this macro with multiple arguments. In this case it'll generate a tuple:
/// ```rust
/// let (pin5, led, tx, a8) = make_pin!(5, led, TX3, A8);
/// ```
/// **This macro is unsafe**
#[macro_export]
macro_rules! make_pin {
    // for each element in the call apply the following match
    ($p:tt, $($tail:tt),+) => {
        (
            make_pin!($p),
            $( // here we iterate element per element so we should not have recursion
                make_pin!($tail)
            ),*
        )
    };
    // ===== special pin =====
    (led) => {
        crate::port::Port::new(crate::port::PortName::C).pin(5)
    };
    // ===== analog pins =====
    (A17) => {
        crate::port::Pin::new(28)
    };
    (A16) => {
        crate::port::Pin::new(27)
    };
    (A15) => {
        crate::port::Pin::new(26)
    };
    (A18) => {
        crate::port::Pin::new(29)
    };
    (A19) => {
        crate::port::Pin::new(30)
    };
    (A20) => {
        crate::port::Pin::new(31)
    };
    (A9) => {
        crate::port::Pin::new(23)
    };
    (A8) => {
        crate::port::Pin::new(22)
    };
    (A7) => {
        crate::port::Pin::new(21)
    };
    (A6) => {
        crate::port::Pin::new(20)
    };
    (A5) => {
        crate::port::Pin::new(19)
    };
    (A4) => {
        crate::port::Pin::new(18)
    };
    (A3) => {
        crate::port::Pin::new(17)
    };
    (A2) => {
        crate::port::Pin::new(16)
    };
    (A1) => {
        crate::port::Pin::new(15)
    };
    (A0) => {
        crate::port::Pin::new(14)
    };
    // ===== I2C port =====
    (SCL0) => {
        crate::port::Pin::new(19)
    };
    (SDA0) => {
        crate::port::Pin::new(18)
    };
    (SCL1) => {
        crate::port::Pin::new(29)
    };
    (SDA1) => {
        crate::port::Pin::new(30)
    };
    // ===== serial port =====
    (RX) => {
        crate::port::Pin::new(3)
    };
    (TX) => {
        crate::port::Pin::new(4)
    };
    (RX1) => {
        crate::port::Pin::new(0)
    };
    (TX1) => {
        crate::port::Pin::new(1)
    };
    (RX2) => {
        crate::port::Pin::new(9)
    };
    (TX2) => {
        crate::port::Pin::new(10)
    };
    (RX3) => {
        crate::port::Pin::new(7)
    };
    (TX3) => {
        crate::port::Pin::new(8)
    };
    // ===== SPI port =====
    (CS) => {
        crate::port::Pin::new(10)
    };
    (DOUT) => {
        crate::port::Pin::new(11)
    };
    (DIN) => {
        crate::port::Pin::new(12)
    };
    (SCK) => {
        crate::port::Pin::new(13)
    };
    // ===== Schematic view =====
    (PTA4) => {
        crate::port::Pin::new(33)
    };
    (PTB18) => {
        crate::port::Pin::new(32)
    };
    (PTE0) => {
        crate::port::Pin::new(31)
    };
    (PTC11) => {
        crate::port::Pin::new(30)
    };
    (PTC10) => {
        crate::port::Pin::new(29)
    };
    (PTC8) => {
        crate::port::Pin::new(28)
    };
    (PTC9) => {
        crate::port::Pin::new(27)
    };
    (PTE1) => {
        crate::port::Pin::new(26)
    };
    (PTB19) => {
        crate::port::Pin::new(25)
    };
    (PTA5) => {
        crate::port::Pin::new(24)
    };
    (PTC2) => {
        crate::port::Pin::new(23)
    };
    (PTC1) => {
        crate::port::Pin::new(22)
    };
    (PTD6) => {
        crate::port::Pin::new(21)
    };
    (PTD5) => {
        crate::port::Pin::new(20)
    };
    (PTB2) => {
        crate::port::Pin::new(19)
    };
    (PTB3) => {
        crate::port::Pin::new(18)
    };
    (PTB1) => {
        crate::port::Pin::new(17)
    };
    (PTB0) => {
        crate::port::Pin::new(16)
    };
    (PTC0) => {
        crate::port::Pin::new(15)
    };
    (PTD1) => {
        crate::port::Pin::new(14)
    };
    (PTC5) => {
        crate::port::Pin::new(13)
    };
    (PTC7) => {
        crate::port::Pin::new(12)
    };
    (PTC6) => {
        crate::port::Pin::new(11)
    };
    (PTC4) => {
        crate::port::Pin::new(10)
    };
    (PTC3) => {
        crate::port::Pin::new(9)
    };
    (PTD3) => {
        crate::port::Pin::new(8)
    };
    (PTD2) => {
        crate::port::Pin::new(7)
    };
    (PTD4) => {
        crate::port::Pin::new(6)
    };
    (PTD7) => {
        crate::port::Pin::new(5)
    };
    (PTA13) => {
        crate::port::Pin::new(4)
    };
    (PTA12) => {
        crate::port::Pin::new(3)
    };
    (PTD0) => {
        crate::port::Pin::new(2)
    };
    (PTB17) => {
        crate::port::Pin::new(1)
    };
    (PTB16) => {
        crate::port::Pin::new(0)
    };
    // ===== digital pins =====
    ($n:expr) => {
        crate::port::Pin::new($n)
    };
}
