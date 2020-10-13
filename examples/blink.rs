#![feature(stdsimd)]
#![no_std]
#![no_main]

use embedded_hal::prelude::*;
use teensy::*;

define_panic! {empty}

#[no_mangle]
fn main() {
    let mut led = unsafe { make_pin!(led).make_gpio().with_output() };

    loop {
        led.try_toggle().unwrap();
        sleep::sleep_ms(500);
    }
}
