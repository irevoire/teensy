#![feature(stdsimd)]
#![no_std]
#![no_main]

use teensy::*;
use teensy::port::Pin;

define_panic!{empty}

#[no_mangle]
fn main() {
    let led : Pin = unsafe { make_pin!(led) };

    let mut led_out = led.make_gpio();

    led.output();

    loop {
        led.toggle();
        sleep::sleep_ms(500);
    }
}
