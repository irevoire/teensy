#![feature(stdsimd)]
#![no_std]
#![no_main]

use teensy::*;

define_panic!(empty);

#[no_mangle]
fn main() {
    let pin = unsafe { port::Port::new(port::PortName::C).pin(5) };

    let mut gpio = pin.make_gpio();

    gpio.output();

    loop {
        gpio.toggle();
        sleep::sleep_ms(500);
    }
}
