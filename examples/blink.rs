#![feature(stdsimd)]
#![no_std]
#![no_main]

use teensy::*;

define_panic!{empty}

#[no_mangle]
fn main() {
    let led = unsafe { make_pin!(led) };

    let mut led = led.make_gpio();

    led.output();

    loop {
        led.toggle();
        sleep::sleep_ms(500);
    }
}
