#![feature(stdsimd)]
#![no_std]
#![no_main]

use teensy::*;

define_panic!(empty);

#[no_mangle]
fn main() {
    let (led, pin) = unsafe { make_pins!(led, 3) };

    let mut led = led.make_gpio();
    let mut pin = pin.make_gpio();

    led.output();
    pin.input();

    loop {
        match pin.read() {
            0 => led.low(),
            _ => led.high(),
        }
        pin.output(); // currently we need to do that to "reset" the pin
        pin.input(); // This is a bug but currently we don't know how to do it better
    }
}
